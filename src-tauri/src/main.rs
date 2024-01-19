// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // TODO: remove

use std::{thread, sync::Arc};

use characters::CharacterKit;
use damage::CharacterStats;
use data::{CharacterDescriptor, RelicSlot, EffectPropertyType};
use data_mappings::RelicSet;
use relics::{Relic, RelicSetKit, ConditionalRelicSetEffects, RelicSetKitParams};

use crate::relics::Permute;
use crate::scans::TEST_SCAN;
use crate::{data::use_character, damage::{Boosts, EnemyConfig}, data_mappings::{Character, LightCone}, promotions::{CharacterState, CharacterSkillState, CharacterTraceState, calculate_character_base_stats, LightConeState}, characters::{apply_minor_trace_effects, jingliu::{Jingliu, JingliuDescriptions}}, lightcones::{i_shall_be_my_own_sword::{IShallBeMyOwnSword, IShallBeMyOwnSwordDesc}, LightConeKit}};

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
#[tauri::command]
#[specta::specta]
fn greet(_name: &str) -> String {
    // let mut boosts = Boosts::default();

    // let jingliu = Jingliu {};
    let character_id = Character::Jingliu;
    let character = use_character(character_id);
    let character_state = CharacterState {
        ascension: 6,
        eidolon: 6,
        level: 80,
        skills: CharacterSkillState {
            basic: 7 - 1,
            skill: 12 - 1,
            ult: 12 - 1,
            talent: 12 - 1,
        },
        traces: CharacterTraceState {
            ability_1: true,
            ability_2: true,
            ability_3: true,
            stat_1: true,
            stat_2: true,
            stat_3: true,
            stat_4: true,
            stat_5: true,
            stat_6: true,
            stat_7: true,
            stat_8: true,
            stat_9: true,
            stat_10: true,
        }
    };

    let light_cone_id = LightCone::IShallBeMyOwnSword;
    let light_cone_state = LightConeState {
        level: 80,
        ascension: 6,
        superimposition: 1 - 1,
    };
    let character_stats = calculate_character_base_stats((character_id, character_state), Some((light_cone_id, light_cone_state)));

    // TODO: Apply light cone effect properly
    // boosts.crit_dmg += 0.2;
    // boosts.all_type_dmg_boost += 0.14 * 3.0;
    // boosts.def_shred += 0.12;

    let kit = Jingliu {
        descriptions: JingliuDescriptions::get(),

        enhanced_state: true,
        hp_drain_pct: 1.0,

        e1_crit_dmg: true,
        e2_skill_buff: true,
    };

    let lc_kit = IShallBeMyOwnSword {
        descriptions: IShallBeMyOwnSwordDesc::get(),
        eclipse_stacks: 3,
    };

    let enemy_config = EnemyConfig {
        count: 1,
        level: 95,
    
        resistance: 0.2,
        elemental_weakness: true,
        weakness_broken: false,
    };

    let all_relics = TEST_SCAN.relics.iter().filter_map(|r| r.to_relic()).collect::<Vec<_>>();

    println!("Relics: {:?}", all_relics.iter().filter(|r| 
        r.slot == RelicSlot::Chest
            // r.main_stat.0 == EffectPropertyType::CriticalChanceBase ||
            // r.main_stat.0 == EffectPropertyType::CriticalDamageBase
        // )
    ).collect::<Vec<_>>(),);

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

    // let special_boosts = Boosts { hp_flat: 814.6000000101048, hp_pct: 11.6, atk_flat: 387.8000000116881, atk_pct: 61.64400001449512, def_flat: 101.0, def_pct: 9.7, spd: 11.0, effect_res: 11.549999999999999, effect_hit_rate: 4.3, crit_rate: 6.943999998625362, crit_dmg: 78.213, break_effect: 5.1, energy_recharge: 0.0, outgoing_healing_boost: 0.0, elemental_dmg_boost: 0.0, all_type_dmg_boost: 1.2200000000000002, extra_vulnerability: 0.0, def_shred: 0.12, res_pen: 0.0 };
    // let test = kit.compute_stat_column(characters::StatColumnType::SkillDamage, &character_state, &character_stats, &special_boosts, &enemy_config);

    let time = std::time::Instant::now();

    let cols = calculate_cols(
        CalculatorParameters {
            character: character.clone(),
            character_kit: Arc::new(kit),
            character_state,
            character_stats,
            light_cone: light_cone_id,
            light_cone_kit: Arc::new(lc_kit),
            light_cone_state,
            enemy_config,
            relic_conditionals: ConditionalRelicSetEffects::default(),
        },
        relics_by_slot.clone()
    );

    let duration = time.elapsed();

    format!("(Checked {} perms in {}s) Results: {:?} {:?}", relics_by_slot.permutations().size(), duration.as_secs_f64(), character_stats, cols)
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

fn calculate_cols(
    // character: (&CharacterDescriptor, &(dyn CharacterKit+Sync), &CharacterState, &CharacterStats), light_cone: (&(dyn LightConeKit+Sync), &LightConeState), enemy_config: &EnemyConfig
    params: CalculatorParameters,
    relics: Vec<Vec<Relic>>,
) -> (Vec<Relic>, Vec<(String, f64)>, (CharacterStats, CharacterStats)) {
    let mut base_boosts = Boosts::default();
    
    // let hands = Relic {
    //     set: RelicSet::GeniusOfBrilliantStars,
    //     slot: RelicSlot::Hands,
    //     level: 15,
    //     main_stat: (EffectPropertyType::AttackDelta, 352.8),
    //     sub_stats: vec![
    //         (EffectPropertyType::HPDelta, 80.0),
    //         (EffectPropertyType::AttackAddedRatio, 0.06)
    //     ]
    // };
    // boosts.atk_flat += 352.8; // Hands Relic TODO: draw the rest of the owl
    // hands.apply(effective_element, &mut boosts);

    apply_minor_trace_effects(&params.character, &params.character_state, &mut base_boosts);
    params.light_cone_kit.apply_base_passives(&params.enemy_config, &params.light_cone_state, &mut base_boosts);
    params.character_kit.apply_base_passives(&params.enemy_config, &params.character_state, &mut base_boosts);

    let mut combat_boosts = Boosts::default();
    params.light_cone_kit.apply_base_combat_passives(&params.enemy_config, &params.light_cone_state, &mut combat_boosts);
    params.character_kit.apply_base_combat_passives(&params.enemy_config, &params.character_state, &mut combat_boosts);

    // for relic in relics.iter() {
    //     relic.apply(effective_element, &mut boosts);
    // }

    let thread_count = 8;
    let batches = relics.permutation_batches(thread_count);

    let mut threads = vec![];
    for tid in 0..thread_count {
        // let my_kit = kit.clone();
        let params = params.clone();
        let relics = relics.clone();
        let cols = params.character_kit.get_stat_columns();
        let batches = batches.clone();
        threads.push(thread::spawn(move || {
            // let mut max: Option<Vec<(String, f64)>> = None;
            // let mut max_relics: Option<Vec<&Relic>> = None;
            // let mut max_boosts: Option<Boosts> = None;

            let mut results = vec![];
            results.reserve(batches[tid].size());

            for relic_perm in relics.enumerated_permutation_subset(&batches[tid]) {
                let mut base_boosts = base_boosts.clone();

                // TODO: Test if using a static map is faster
                // let mut sets = HashMap::new();
                // sets.reserve(relic_perm.len()); // Does this micro-optimization even matter?

                // let mut active_sets = vec![];

                // for &relic in relic_perm.iter() {
                //     relic.apply(params.character.element, &mut base_boosts);
                //     let p = *sets.entry(relic.set).and_modify(|x| *x += 1).or_insert(1);
                //     if p == 2 {
                //         if let Some(effect) = relic.set.get_2p_effect() { active_sets.push(effect); }
                //     } else if p == 4 {
                //         if let Some(effect) = relic.set.get_4p_effect() { active_sets.push(effect); }
                //     }
                // }
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

                let cols: Vec<(String, f64)> = cols.iter().map(|&column_type| {
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

                    (column_type.to_name().to_owned(), params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &skill_boosts, &params.enemy_config))
                }).collect();

                // if let Some(maxv) = &max {
                //     // for (i, (name, value)) in cols.iter().enumerate() {
                //     if cols[0].1 > maxv[0].1 {
                //         max = Some(cols);
                //         max_relics = Some(relic_perm);
                //         // max_boosts = Some(boosts);
                //     }
                // } else {
                //     max = Some(cols);
                //     max_relics = Some(relic_perm);
                //     // max_boosts = Some(boosts);
                // }

                // if cols[0].1 > 80000.0 {
                results.push((
                    relic_perm.into_iter().map(|(_, i)| i).collect::<Vec<usize>>(), 
                    // relic_perm.into_iter().map(|x|)
                    cols, 
                    (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
                ));
                // }
            }

            return results;
        }));
    }

    let mut max: Option<Vec<(String, f64)>> = None;
    let mut max_relics: Option<Vec<Relic>> = None;
    let mut max_boosts: Option<(CharacterStats, CharacterStats)> = None;

    for thread in threads {
        let results = thread.join().unwrap();

        for (relic_perm, cols, boosts) in results {
            let resolved_relics = relic_perm.into_iter().enumerate().map(|(i, j)| relics[i][j].clone()).collect::<Vec<_>>();

            if let Some(maxv) = &max {
                // for (i, (name, value)) in cols.iter().enumerate() {
                if cols[0].1 > maxv[0].1 {
                    max = Some(cols);
                    max_relics = Some(resolved_relics);
                    max_boosts = Some(boosts);
                }
            } else {
                max = Some(cols);
                max_relics = Some(resolved_relics);
                max_boosts = Some(boosts);
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

    return (max_relics.unwrap(), max.unwrap(), max_boosts.unwrap());
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![greet]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.gen.ts");

        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
