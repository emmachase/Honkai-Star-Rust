// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(dead_code)] // TODO: remove

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{thread, sync::Arc};

use characters::{CharacterKit, CharacterConfig, StatColumnType, StatColumnDesc};
use damage::CharacterStats;
use data::{CharacterDescriptor, RelicSlot, EffectPropertyType};
use data_mappings::RelicSet;
use lightcones::LightConeConfig;
use relics::{Relic, RelicSetKit, ConditionalRelicSetEffects, RelicSetKitParams};
use scans::KelZScan;
use serde::{Serialize, Deserialize};
use specta::Type;

use crate::relics::Permute;
use crate::{data::use_character, damage::{Boosts, EnemyConfig}, promotions::{CharacterState, calculate_character_base_stats, LightConeState}, characters::apply_minor_trace_effects, lightcones::LightConeKit};

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
    relics: Vec<Relic>,
    character_cfg: CharacterConfig,
    character_state: CharacterState,
    light_cone_cfg: LightConeConfig,
    light_cone_state: LightConeState,
    enemy_config: EnemyConfig
) -> SortResultsSerde {
    let character_id = character_cfg.get_character_id();
    let character = use_character(character_id);

    let light_cone_id = light_cone_cfg.get_light_cone_id();

    let character_stats = calculate_character_base_stats((character_id, character_state), Some((light_cone_id, light_cone_state)));

    let kit = character_cfg.get_kit();
    let lc_kit = light_cone_cfg.get_kit();

    let relics_by_slot = vec![
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Head).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Hands).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Chest).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Feet).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::PlanarSphere).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::LinkRope).collect::<Vec<_>>(),
    ];

    let time = std::time::Instant::now();

    let cols = calculate_cols(
        CalculatorParameters {
            character: character.clone(),
            character_kit: Arc::from(kit),
            character_state,
            character_stats,
            light_cone_kit: Arc::from(lc_kit),
            light_cone_state,
            enemy_config,
            relic_conditionals: ConditionalRelicSetEffects::default(),
        },
        relics_by_slot.clone()
    );

    let duration = time.elapsed();

    println!("Checked {} perms in {}s", relics_by_slot.permutations().size(), duration.as_secs_f64());

    SortResultsSerde::from((cols, &relics_by_slot))
}

#[tauri::command(async)]
#[specta::specta]
fn parse_kelz(
    scan: String
) -> Result<Vec<Relic>, String> {
    let scan: KelZScan = serde_json::from_str(scan.as_str()).map_err(|e| format!("{}", e))?;
    Ok(scan.relics.into_iter().filter_map(|r| r.to_relic()).collect::<Vec<_>>())
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
    light_cone_kit: Arc<dyn LightConeKit+Sync+Send>,
    light_cone_state: LightConeState,
    enemy_config: EnemyConfig,
    relic_conditionals: ConditionalRelicSetEffects
}

#[derive(Debug, PartialEq, Clone)]
struct CalculatorResult {
    relic_perm: Vec<usize>, // Relic id per slot
    cols: Vec<(String, f64)>,
    calculated_stats: (CharacterStats, CharacterStats), // (Base stats, Combat stats)
    comparable: f64
}

impl Comparable for CalculatorResult {
    fn get_comparable(&self) -> f64 {
        self.comparable
    }
}

#[derive(Debug, PartialEq)]
struct PreCalculatorResult<'a> {
    cols: &'a Vec<(StatColumnType, f64)>,
    calculated_stats: &'a (CharacterStats, CharacterStats) // (Base stats, Combat stats)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct ResolvedCalculatorResult {
    pub relic_perm: Vec<Relic>,
    pub cols: Vec<(String, f64)>,
    pub calculated_stats: (CharacterStats, CharacterStats) // (Base stats, Combat stats)
}

impl From<(CalculatorResult, &Vec<Vec<Relic>>)> for ResolvedCalculatorResult {
    fn from((result, relics_by_slot): (CalculatorResult, &Vec<Vec<Relic>>)) -> Self {
        let resolved_relics = result.relic_perm.into_iter().enumerate().map(|(i, j)| relics_by_slot[i][j].clone()).collect::<Vec<_>>();
        Self {
            relic_perm: resolved_relics,
            cols: result.cols,
            calculated_stats: result.calculated_stats
        }
    }
}

#[derive(Debug, Default)]
struct SortResults {
    pub hp: BinaryHeap<Reverse<CalculatorResult>>,
    pub atk: BinaryHeap<Reverse<CalculatorResult>>,
    pub def: BinaryHeap<Reverse<CalculatorResult>>,
    pub spd: BinaryHeap<Reverse<CalculatorResult>>,
    pub effect_res: BinaryHeap<Reverse<CalculatorResult>>,
    pub crit_rate: BinaryHeap<Reverse<CalculatorResult>>,
    pub crit_dmg: BinaryHeap<Reverse<CalculatorResult>>,
    pub break_effect: BinaryHeap<Reverse<CalculatorResult>>,
    pub energy_recharge: BinaryHeap<Reverse<CalculatorResult>>,
    pub outgoing_healing_boost: BinaryHeap<Reverse<CalculatorResult>>,
    pub elemental_dmg_bonus: BinaryHeap<Reverse<CalculatorResult>>,
    pub effect_hit_rate: BinaryHeap<Reverse<CalculatorResult>>,

    pub cols: Vec<(StatColumnType, BinaryHeap<Reverse<CalculatorResult>>)>,
}

trait Comparable {
    fn get_comparable(&self) -> f64;
}

trait AddToHeap<H, I, P> {
    fn add_to_heap(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, comparable: f64);
}

fn eval_presult(relic_perm: &[(&Relic, usize); 6], item: &PreCalculatorResult, comparable: f64) -> CalculatorResult {
    let relic_perm = relic_perm.into_iter().map(|(_, i)| *i).collect::<Vec<usize>>();
    let cols = item.cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>();
    let calculated_stats = item.calculated_stats.clone();

    CalculatorResult {
        relic_perm,
        cols,
        calculated_stats,
        comparable
    }
}

fn clone_maker(_: &Vec<usize>, item: &CalculatorResult, _: f64) -> CalculatorResult {
    item.clone()
}

impl<H: PartialEq+Ord+Comparable, I: PartialEq, P> AddToHeap<H, I, P> for BinaryHeap<Reverse<H>> {
    fn add_to_heap(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, comparable: f64) { // maker: fn(P, &I, f64) -> H, comparator: fn(&H, &I) -> Ordering) {
        let cur_min = self.peek();

        if self.len() < top_k || cur_min.unwrap().0.get_comparable() < comparable {
            let result = maker(relic_perm, item, comparable);

            if self.len() >= top_k {
                self.pop(); // Remove the smallest element
            }

            self.push(Reverse(result));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SortResultsSerde {
    pub hp: Vec<ResolvedCalculatorResult>,
    pub atk: Vec<ResolvedCalculatorResult>,
    pub def: Vec<ResolvedCalculatorResult>,
    pub spd: Vec<ResolvedCalculatorResult>,
    pub effect_res: Vec<ResolvedCalculatorResult>,
    pub crit_rate: Vec<ResolvedCalculatorResult>,
    pub crit_dmg: Vec<ResolvedCalculatorResult>,
    pub break_effect: Vec<ResolvedCalculatorResult>,
    pub energy_recharge: Vec<ResolvedCalculatorResult>,
    pub outgoing_healing_boost: Vec<ResolvedCalculatorResult>,
    pub elemental_dmg_bonus: Vec<ResolvedCalculatorResult>,
    pub effect_hit_rate: Vec<ResolvedCalculatorResult>,

    pub cols: Vec<(String, Vec<ResolvedCalculatorResult>)>,
}

impl From<(SortResults, &Vec<Vec<Relic>>)> for SortResultsSerde {
    fn from((sort, relics_by_slot): (SortResults, &Vec<Vec<Relic>>)) -> Self {
        Self {
            hp: sort.hp.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            atk: sort.atk.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            def: sort.def.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            spd: sort.spd.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            effect_res: sort.effect_res.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            crit_rate: sort.crit_rate.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            crit_dmg: sort.crit_dmg.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            break_effect: sort.break_effect.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            energy_recharge: sort.energy_recharge.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            outgoing_healing_boost: sort.outgoing_healing_boost.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            elemental_dmg_bonus: sort.elemental_dmg_bonus.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            effect_hit_rate: sort.effect_hit_rate.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),

            cols: sort.cols.into_iter().map(|(column_type, heap)| {
                (column_type.to_name().to_owned(), heap.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect())
            }).collect()
        }
    }
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

fn calculate_cols(
    params: CalculatorParameters,
    relics: Vec<Vec<Relic>>,
) -> SortResults {
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
        let kit_cols = params.character_kit.get_stat_columns(&params.enemy_config);
        let batches = batches.clone();
        threads.push(thread::spawn(move || {
            let mut all_results = SortResults::default();

            // Initialize the dynamic columns
            for k in kit_cols.iter() {
                all_results.cols.push((k.column_type, BinaryHeap::new()));
            }

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
                kit_cols.iter().for_each(|StatColumnDesc { column_type, hit_splits }| {
                    let mut skill_boosts = total_combat_boosts.clone();
                    let column_type = *column_type;

                    params.light_cone_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.light_cone_state, &mut skill_boosts);
                    params.character_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.character_state, &mut skill_boosts);
                    active_sets.apply_stat_type_conditionals(RelicSetKitParams {
                        enemy_config: &params.enemy_config,
                        conditionals: &params.relic_conditionals,
                        character_stats: &params.character_stats,
                        character_element: params.character.element,
                        boosts: &mut skill_boosts,
                    }, column_type);

                    let mut col_total = 0.0;
                    for split in hit_splits.iter().enumerate() {
                        active_sets.apply_inter_hit_effects(split, RelicSetKitParams {
                            enemy_config: &params.enemy_config,
                            conditionals: &params.relic_conditionals,
                            character_stats: &params.character_stats,
                            character_element: params.character.element,
                            boosts: &mut skill_boosts,
                        }, column_type);

                        col_total += params.character_kit.compute_stat_column(column_type, split, &params.character_state, &params.character_stats, &skill_boosts, &params.enemy_config);
                    }

                    cols.push((column_type, col_total));
                });

                let presult = PreCalculatorResult {
                    cols: &cols,
                    calculated_stats: &(params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
                };

                { // TODO: This is a bit of a mess
                    all_results.hp.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.hp);
                    all_results.atk.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.atk);
                    all_results.def.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.def);
                    all_results.spd.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.spd);
                    all_results.effect_res.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.effect_res);
                    all_results.crit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.crit_rate);
                    all_results.crit_dmg.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.crit_dmg);
                    all_results.break_effect.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.break_effect);
                    all_results.energy_recharge.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.energy_recharge);
                    all_results.outgoing_healing_boost.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.outgoing_healing_boost);
                    all_results.elemental_dmg_bonus.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.elemental_dmg_bonus);
                    all_results.effect_hit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.effect_hit_rate);

                    for (i, col) in cols.iter().enumerate() {
                        all_results.cols[i].1.add_to_heap(top_k, &presult, &relic_perm, eval_presult, col.1);
                    }
                }
            }

            return all_results;
        }));
    }

    let mut combined_results = SortResults::default();

    for thread in threads {
        let results = thread.join().unwrap();

        // TODO: Do this some other way, or at least extract it
        for Reverse(result) in results.hp { combined_results.hp.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.atk { combined_results.atk.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.def { combined_results.def.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.spd { combined_results.spd.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.effect_res { combined_results.effect_res.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.crit_rate { combined_results.crit_rate.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.crit_dmg { combined_results.crit_dmg.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.break_effect { combined_results.break_effect.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.energy_recharge { combined_results.energy_recharge.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.outgoing_healing_boost { combined_results.outgoing_healing_boost.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.elemental_dmg_bonus { combined_results.elemental_dmg_bonus.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        for Reverse(result) in results.effect_hit_rate { combined_results.effect_hit_rate.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }

        for (i, col) in results.cols.into_iter().enumerate() {
            if combined_results.cols.len() <= i {
                combined_results.cols.push((col.0, BinaryHeap::new()));
            }

            for Reverse(result) in col.1 { combined_results.cols[i].1.add_to_heap(top_k, &result, &result.relic_perm, clone_maker, result.comparable) }
        }
    }

    return combined_results
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![prank_him_john, parse_kelz]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.gen.ts");

        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
        .invoke_handler(tauri::generate_handler![prank_him_john, parse_kelz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
