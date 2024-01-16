// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // TODO: remove

use std::{thread, sync::Arc};

use characters::common::CharacterKit;
use damage::CharacterStats;
use data::CharacterDescriptor;

use crate::{data::use_character, damage::{Boosts, Level, Ascension, EnemyConfig}, data_mappings::{Character, LightCone}, promotions::{CharacterState, CharacterSkillState, CharacterTraceState, calculate_character_base_stats, LightConeState}, characters::{common::apply_minor_trace_effects, jingliu::{Jingliu, JingliuDescriptions}}, lightcones::{i_shall_be_my_own_sword::{IShallBeMyOwnSword, IShallBeMyOwnSwordDesc}, common::LightConeKit}};

#[path = "data.gen.rs"]
mod data_mappings;

mod damage;
mod data;
mod util;
mod promotions;
mod characters;
mod lightcones;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
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

    let mut enemy_config = EnemyConfig {
        count: 1,
        level: 95,
    
        resistance: 0.2,
        elemental_weakness: true,
        weakness_broken: false,
    };

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
        }
    );

    format!("eNo relics: {:?} {:?}", character_stats, cols)
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
}

fn calculate_cols(
    // character: (&CharacterDescriptor, &(dyn CharacterKit+Sync), &CharacterState, &CharacterStats), light_cone: (&(dyn LightConeKit+Sync), &LightConeState), enemy_config: &EnemyConfig
    params: CalculatorParameters
) -> Vec<(String, f64)> {
    let mut boosts = Boosts::default();
    
    boosts.atk_flat += 352.8; // Hands Relic TODO: draw the rest of the owl
    
    apply_minor_trace_effects(&params.character, &params.character_state, &mut boosts);
    params.light_cone_kit.apply_static_passives(&params.enemy_config, &params.light_cone_state, &mut boosts);
    params.character_kit.apply_static_passives(&params.enemy_config, &params.character_state, &mut boosts);

    // let mut threads = vec![];
    // for tid in 0..16 {
    //     // let my_kit = kit.clone();
    //     let params = params.clone();
    //     let cols = params.character_kit.get_stat_columns();
    //     threads.push(thread::spawn(move || {
    //         for _ in 0..20_000_000/16 {
    //             let r: Vec<(String, f64)> = cols.iter().map(|&column_type| {
    //                 let mut special_boosts = boosts.clone();
    //                 params.light_cone_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.light_cone_state, &mut special_boosts);
    //                 params.character_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.character_state, &mut special_boosts);

    //                 (column_type.to_name().to_owned(), params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &special_boosts, &params.enemy_config))
    //             }).collect();
    //         }
    //     }));
    // }

    // for thread in threads {
    //     thread.join().unwrap();
    // }

    let cols = params.character_kit.get_stat_columns();
    cols.iter().map(|&column_type| {
        let mut special_boosts = boosts.clone();
        params.light_cone_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.light_cone_state, &mut special_boosts);
        params.character_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.character_state, &mut special_boosts);

        (column_type.to_name().to_owned(), params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &special_boosts, &params.enemy_config))
    }).collect()
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
