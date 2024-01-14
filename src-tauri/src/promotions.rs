use serde::{Deserialize, Serialize};

use crate::{data_mappings::{Character, LightCone}, damage::{Level, Ascension, CharacterStats, Eidolon}, data::{Promotions, PromotionStepSpec, use_character_promotions, use_character, use_light_cone_promotions, CharacterDescriptor, use_character_trace_node, EffectPropertyType}, characters::common::CharacterTraceIds};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct CharacterSkillState {
    pub basic: u8,
    pub skill: u8,
    pub ult: u8,
    pub talent: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct CharacterTraceState {
    pub ability_1: bool,
    pub ability_2: bool,
    pub ability_3: bool,
    pub stat_1: bool,
    pub stat_2: bool,
    pub stat_3: bool,
    pub stat_4: bool,
    pub stat_5: bool,
    pub stat_6: bool,
    pub stat_7: bool,
    pub stat_8: bool,
    pub stat_9: bool,
    pub stat_10: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct CharacterState {
    pub level: Level,
    pub ascension: Ascension,
    pub eidolon: Eidolon,
    pub skills: CharacterSkillState,
    pub traces: CharacterTraceState,
}

fn promote<Spec>(promotions: &Promotions<Spec>, level: Level, ascension: Ascension, stat_fn: fn(&Spec) -> PromotionStepSpec) -> f64 {
    let spec = &promotions.values[ascension as usize - 1];
    let spec = stat_fn(spec);

    return spec.base + (level as f64 - 1.0) * spec.step;
}

// fn add_trace_effects(trace_id: &str, stats: &mut CharacterStats) {
//     let trace = use_character_trace_node(trace_id);
//     for effect in &trace.levels[0].properties {
//         match effect.property_type {
//             EffectPropertyType::HPDelta                   => unreachable!("HPDelta is not a trace effect"),
//             EffectPropertyType::AttackDelta               => unreachable!("AttackDelta is not a trace effect"),
//             EffectPropertyType::DefenceDelta              => unreachable!("DefenceDelta is not a trace effect"),
//             EffectPropertyType::SpeedDelta                => todo!(),
//             EffectPropertyType::HPAddedRatio              => todo!(),
//             EffectPropertyType::AttackAddedRatio          => todo!(),
//             EffectPropertyType::DefenceAddedRatio         => todo!(),
//             EffectPropertyType::CriticalChanceBase        => todo!(),
//             EffectPropertyType::CriticalDamageBase        => todo!(),
//             EffectPropertyType::HealRatioBase             => todo!(),
//             EffectPropertyType::StatusProbabilityBase     => todo!(),
//             EffectPropertyType::PhysicalAddedRatio        => todo!(),
//             EffectPropertyType::FireAddedRatio            => todo!(),
//             EffectPropertyType::IceAddedRatio             => todo!(),
//             EffectPropertyType::ThunderAddedRatio         => todo!(),
//             EffectPropertyType::WindAddedRatio            => todo!(),
//             EffectPropertyType::QuantumAddedRatio         => todo!(),
//             EffectPropertyType::ImaginaryAddedRatio       => todo!(),
//             EffectPropertyType::BreakDamageAddedRatioBase => todo!(),
//             EffectPropertyType::SPRatioBase               => todo!(),
//             EffectPropertyType::StatusResistanceBase      => todo!(),
//         }
//     }
// }

// fn add_all_trace_effects(character: &'static CharacterDescriptor, character_state: &CharacterState, stats: &mut CharacterStats) {
//     let trace_ids = CharacterTraceIds::from_character(character);

//     if character_state.traces.ability_1 {  }
// }

pub fn calculate_character_base_stats(character: (Character, CharacterState), light_cone: Option<(LightCone, Level, Ascension)>) -> CharacterStats {
    let (character, character_state) = character;
    
    let character_promotions = use_character_promotions(character);
    let character = use_character(character);

    let mut stats = CharacterStats {
        level: character_state.level,
        ascension: character_state.ascension,
        element: character.element,

        hp        : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.hp),
        atk       : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.atk),
        def       : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.def),
        spd       : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.spd),
        crit_rate : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.crit_rate),
        crit_dmg  : promote(&character_promotions, character_state.level, character_state.ascension, |s| s.crit_dmg),

        break_effect: 0.0,
        effect_res: 0.0,
        energy_recharge: 0.0,
        outgoing_healing_boost: 0.0,
        elemental_dmg_bonus: 0.0,
    };

    if let Some(light_cone) = light_cone {
        let (light_cone, light_cone_level, light_cone_ascension) = light_cone;

        let light_cone_promotions = use_light_cone_promotions(light_cone);

        stats.hp  += promote(&light_cone_promotions, light_cone_level, light_cone_ascension, |s| s.hp);
        stats.atk += promote(&light_cone_promotions, light_cone_level, light_cone_ascension, |s| s.atk);
        stats.def += promote(&light_cone_promotions, light_cone_level, light_cone_ascension, |s| s.def);
    }

    return stats;
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn test_calculate_character_base_stats() {
        let character = (Character::Jingliu, CharacterState {
            level: 67 as Level, 
            ascension: 6 as Ascension,
            eidolon: 0 as Eidolon,
            skills: CharacterSkillState {
                basic: 6,
                skill: 6,
                ult: 6,
                talent: 6,
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
            },
        });
        let light_cone = Some((LightCone::IShallBeMyOwnSword, 56 as Level, 5 as Ascension));
        let stats = calculate_character_base_stats(character, light_cone);
        assert_float_relative_eq!(stats.hp,  1230.768 + 805.2);
        assert_float_relative_eq!(stats.atk, 582.12   + 402.6);
        assert_float_relative_eq!(stats.def, 415.80   + 274.5);
    }
}
