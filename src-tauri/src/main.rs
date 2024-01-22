// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // TODO: remove

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::RwLock;
use std::{thread, sync::Arc};

use characters::{CharacterKit, CharacterConfig, StatColumnType};
use damage::CharacterStats;
use data::{CharacterDescriptor, RelicSlot, EffectPropertyType};
use data_mappings::RelicSet;
use lightcones::LightConeConfig;
use relics::{Relic, RelicSetKit, ConditionalRelicSetEffects, RelicSetKitParams};
use serde::{Serialize, Deserialize};
use specta::Type;
use tauri::State;

use crate::relics::Permute;
use crate::scans::TEST_SCAN;
use crate::{data::use_character, damage::{Boosts, EnemyConfig}, data_mappings::LightCone, promotions::{CharacterState, calculate_character_base_stats, LightConeState}, characters::apply_minor_trace_effects, lightcones::LightConeKit};

#[path = "data.gen.rs"]
mod data_mappings;
mod scans;

mod damage;
mod data;
mod util;
mod promotions;
mod characters;
mod lightcones;
mod relics;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
#[specta::specta]
fn prank_him_john(
    storage: State<Storage>,
    character_cfg: CharacterConfig, 
    character_state: CharacterState, 
    light_cone_cfg: LightConeConfig, 
    light_cone_state: LightConeState, 
    enemy_config: EnemyConfig
) -> String {
    let character_id = character_cfg.get_character_id();
    let character = use_character(character_id);

    let light_cone_id = light_cone_cfg.get_light_cone_id();

    let character_stats = calculate_character_base_stats((character_id, character_state), Some((light_cone_id, light_cone_state)));

    let kit = character_cfg.get_kit();
    let lc_kit = light_cone_cfg.get_kit();

    let all_relics = storage.relics.read().unwrap();

    let relics_by_slot = vec![
        all_relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Head).collect::<Vec<_>>(),
        all_relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Hands).collect::<Vec<_>>(),
        all_relics.clone().into_iter().filter(|r| 
            r.slot == RelicSlot::Chest && (
                r.main_stat.0 == EffectPropertyType::CriticalChanceBase ||
                r.main_stat.0 == EffectPropertyType::CriticalDamageBase
            )
        ).collect::<Vec<_>>(),
        all_relics.clone().into_iter().filter(|r| 
            r.slot == RelicSlot::Feet && (
                r.main_stat.0 == EffectPropertyType::SpeedDelta
            )
        ).collect::<Vec<_>>(),
        all_relics.clone().into_iter().filter(|r| 
            r.slot == RelicSlot::PlanarSphere && (
                r.main_stat.0 == EffectPropertyType::IceAddedRatio ||
                r.main_stat.0 == EffectPropertyType::AttackAddedRatio
            )
        ).collect::<Vec<_>>(),
        all_relics.clone().into_iter().filter(|r| r.slot == RelicSlot::LinkRope).collect::<Vec<_>>(),
    ];

    println!("Relics: {:?}", relics_by_slot);

    let time = std::time::Instant::now();

    let cols = calculate_cols(
        CalculatorParameters {
            character: character.clone(),
            character_kit: Arc::from(kit),
            character_state,
            character_stats,
            light_cone: light_cone_id,
            light_cone_kit: Arc::from(lc_kit),
            light_cone_state,
            enemy_config,
            relic_conditionals: ConditionalRelicSetEffects::default(),
        },
        relics_by_slot.clone()
    );

    let duration = time.elapsed();

    format!("(Checked {} perms in {}s) Results: {:?} {:?}", relics_by_slot.permutations().size(), duration.as_secs_f64(), character_stats, cols)
}

#[tauri::command(async)]
#[specta::specta]
fn get_filtered_relic_count(
    storage: State<Storage>,
    filters: RelicFilters
) {

}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
struct RelicFilters {
    chest        : Vec<EffectPropertyType>,
    feet         : Vec<EffectPropertyType>,
    planar_sphere: Vec<EffectPropertyType>,
    link_rope    : Vec<EffectPropertyType>,
}

#[derive(Clone)]
struct CalculatorParameters {
    character: CharacterDescriptor,
    character_kit: Arc<dyn CharacterKit+Sync+Send>,
    character_state: CharacterState,
    character_stats: CharacterStats,
    light_cone: LightCone,
    light_cone_kit: Arc<dyn LightConeKit+Sync+Send>,
    light_cone_state: LightConeState,
    enemy_config: EnemyConfig,
    relic_conditionals: ConditionalRelicSetEffects
}

#[derive(PartialEq)]
struct CalculatorResult {
    relic_perm: Vec<usize>, // Relic id per slot
    cols: Vec<(String, f64)>,
    calculated_stats: (CharacterStats, CharacterStats) // (Base stats, Combat stats)
}

impl Eq for CalculatorResult {}

impl PartialOrd for CalculatorResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_comparable().partial_cmp(&other.get_comparable()) // TODO
    }
}

impl Ord for CalculatorResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("you done goofed")
    }
}

impl CalculatorResult {
    fn get_comparable(&self) -> f64 {
        self.cols[0].1
    }
}

fn calculate_cols(
    params: CalculatorParameters,
    relics: Vec<Vec<Relic>>,
) -> (Vec<Relic>, Vec<(String, f64)>, (CharacterStats, CharacterStats)) {
    let mut base_boosts = Boosts::default();
    
    apply_minor_trace_effects(&params.character, &params.character_state, &mut base_boosts);
    params.light_cone_kit.apply_base_passives(&params.enemy_config, &params.light_cone_state, &mut base_boosts);
    params.character_kit.apply_base_passives(&params.enemy_config, &params.character_state, &mut base_boosts);

    let mut combat_boosts = Boosts::default();
    params.light_cone_kit.apply_base_combat_passives(&params.enemy_config, &params.light_cone_state, &mut combat_boosts);
    params.character_kit.apply_base_combat_passives(&params.enemy_config, &params.character_state, &mut combat_boosts);

    let thread_count = 16;
    let batches = relics.permutation_batches(thread_count);

    let top_k = 100; // TODO: Configurable

    let mut threads = vec![];
    for tid in 0..thread_count {
        let params = params.clone();
        let relics = relics.clone();
        let kit_cols = params.character_kit.get_stat_columns();
        let batches = batches.clone();
        threads.push(thread::spawn(move || {
            let mut results: BinaryHeap<Reverse<CalculatorResult>> = BinaryHeap::new(); // Reverse gives us a min-heap
            results.reserve(top_k);

            let mut cols: Vec<(StatColumnType, f64)> = Vec::new();
            cols.reserve(3); // TODO

            for relic_perm in relics.enumerated_permutation_subset(&batches[tid]) {
                let mut base_boosts = base_boosts.clone();

                let mut sets = [0u8; RelicSet::COUNT];
                let mut active_sets = [None, None, None];
                let mut set_index = 0;
                for (&ref relic, _) in relic_perm.iter() {
                    relic.apply(params.character.element, &mut base_boosts);

                    sets[relic.set as usize] += 1;
                    let p = sets[relic.set as usize];
                    
                    if p == 2 {
                        if let Some(effect) = relic.set.get_2p_effect() { 
                            active_sets[set_index] = Some(effect); 
                            set_index += 1;
                        }
                    } else if p == 4 {
                        if let Some(effect) = relic.set.get_4p_effect() { 
                            active_sets[set_index] = Some(effect); 
                            set_index += 1;
                        }
                    }
                }

                active_sets.apply_base_passives(RelicSetKitParams {
                    enemy_config: &params.enemy_config, 
                    conditionals: &params.relic_conditionals,
                    character_stats: &params.character_stats, 
                    character_element: params.character.element, 
                    boosts: &mut base_boosts,
                });

                let mut total_combat_boosts = base_boosts + combat_boosts;
                params.light_cone_kit.apply_common_conditionals(&params.enemy_config, &params.light_cone_state, &mut total_combat_boosts);
                params.character_kit.apply_common_conditionals(&params.enemy_config, &params.character_state, &mut total_combat_boosts);
                active_sets.apply_common_conditionals(RelicSetKitParams {
                    enemy_config: &params.enemy_config, 
                    conditionals: &params.relic_conditionals,
                    character_stats: &params.character_stats, 
                    character_element: params.character.element, 
                    boosts: &mut total_combat_boosts,
                });

                cols.clear();
                kit_cols.iter().for_each(|&column_type| {
                    let mut skill_boosts = total_combat_boosts.clone();
                    params.light_cone_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.light_cone_state, &mut skill_boosts);
                    params.character_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.character_state, &mut skill_boosts);
                    active_sets.apply_stat_type_conditionals(RelicSetKitParams {
                        enemy_config: &params.enemy_config, 
                        conditionals: &params.relic_conditionals,
                        character_stats: &params.character_stats, 
                        character_element: params.character.element, 
                        boosts: &mut skill_boosts,
                    }, column_type);

                    cols.push((column_type, params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &skill_boosts, &params.enemy_config)));
                });

                

                let cur_min = results.peek();
                if cur_min.is_none() || cur_min.unwrap().0.get_comparable() < cols[0].1 { // result.get_comparable() {
                    let result = CalculatorResult {
                        relic_perm: relic_perm.into_iter().map(|(_, i)| i).collect::<Vec<usize>>(), 
                        cols: cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>(),
                        calculated_stats: (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
                    };

                    if results.len() >= top_k {
                        results.pop(); // Remove the smallest element
                    }
                    
                    results.push(Reverse(result));
                }
            }

            return results;
        }));
    }

    let mut max: Option<Vec<(String, f64)>> = None;
    let mut max_relics: Option<Vec<Relic>> = None;
    let mut max_stats: Option<(CharacterStats, CharacterStats)> = None;

    for thread in threads {
        let results = thread.join().unwrap();

        for Reverse(CalculatorResult { relic_perm, cols, calculated_stats }) in results {
            let resolved_relics = relic_perm.into_iter().enumerate().map(|(i, j)| relics[i][j].clone()).collect::<Vec<_>>();

            if let Some(maxv) = &max {
                // for (i, (name, value)) in cols.iter().enumerate() {
                if cols[0].1 > maxv[0].1 {
                    max = Some(cols);
                    max_relics = Some(resolved_relics);
                    max_stats = Some(calculated_stats);
                }
            } else {
                max = Some(cols);
                max_relics = Some(resolved_relics);
                max_stats = Some(calculated_stats);
            }
        }
    }

    
    // let mut max_boosts: Option<Boosts> = None;

    // for relic_perm in relics.permutations() {
    //     let mut boosts = boosts.clone();
    //     for &relic in relic_perm.iter() {
    //         relic.apply(params.character.element, &mut boosts);
    //     }

    //     let cols = params.character_kit.get_stat_columns();
    //     let cols: Vec<(String, f64, Boosts)> = cols.iter().map(|&column_type| {
    //         let mut special_boosts = boosts.clone();
    //         params.light_cone_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.light_cone_state, &mut special_boosts);
    //         params.character_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.character_state, &mut special_boosts);

    //         (column_type.to_name().to_owned(), params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &special_boosts, &params.enemy_config), special_boosts)
    //     }).collect();


    // }

    return (max_relics.unwrap(), max.unwrap(), max_stats.unwrap());
}

struct Storage {
    relics: RwLock<Vec<Relic>>
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![prank_him_john, get_filtered_relic_count]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.gen.ts");

        specta_builder.into_plugin()
    };

    let all_relics = TEST_SCAN.relics.iter().filter_map(|r| r.to_relic()).collect::<Vec<_>>();

    tauri::Builder::default()
        .manage(Storage { relics: RwLock::new(all_relics) })
        .plugin(specta_builder)
        .invoke_handler(tauri::generate_handler![prank_him_john, get_filtered_relic_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
