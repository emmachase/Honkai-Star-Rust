// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // TODO: remove

use characters::common::CharacterKit;

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
    let mut boosts = Boosts::default();

    // let jingliu = Jingliu {};
    let character_id = Character::Jingliu;
    let character = use_character(character_id);
    let character_state = CharacterState {
        ascension: 6,
        eidolon: 0,
        level: 80,
        skills: CharacterSkillState {
            basic: 6 - 1,
            skill: 10 - 1,
            ult: 10 - 1,
            talent: 10 - 1,
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
    apply_minor_trace_effects(character, &character_state, &mut boosts);

    let light_cone_id = LightCone::IShallBeMyOwnSword;
    let light_cone_state = LightConeState {
        level: 80,
        ascension: 6,
        superimposition: 1 - 1,
    };
    let stats = calculate_character_base_stats((character_id, character_state), Some((light_cone_id, light_cone_state)));

    // TODO: Apply light cone effect properly
    // boosts.crit_dmg += 0.2;
    // boosts.all_type_dmg_boost += 0.14 * 3.0;
    // boosts.def_shred += 0.12;

    boosts.atk_flat += 352.8; // Hands Relic

    let kit = Jingliu {
        descriptions: JingliuDescriptions::get(),

        enhanced_state: true,
        hp_drain_pct: 1.0,
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

    lc_kit.apply_static_passives(&enemy_config, &light_cone_state, &mut boosts);
    kit.apply_static_passives(&enemy_config, &character_state, &mut boosts);

    let cols = kit.get_stat_columns();
    let cols = cols.iter().map(|c| {
        let mut special_boosts = boosts.clone();
        lc_kit.apply_conditional_passives(&enemy_config, c.stat_type, &light_cone_state, &mut special_boosts);
        kit.apply_conditional_passives(&enemy_config, c.stat_type, &character_state, &mut special_boosts);

        (c.stat_type.to_name(), (c.computer)(&kit, &character_state, &stats, &special_boosts, &enemy_config))
    }).collect::<Vec<_>>();

    format!("No relics: {:?} {:?}", stats, cols)
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
