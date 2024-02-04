use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{data_mappings::{Character, LightCone}, damage::{Level, Ascension, CharacterStats, Eidolon, Superimposition}, data::{Promotions, PromotionStepSpec, use_character_promotions, use_character, use_light_cone_promotions}};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type)]
pub struct CharacterSkillState {
    // These skill levels should be 0-indexed (need to convert from 1-indexed when reading from scanner)
    pub basic: u8,
    pub skill: u8,
    pub ult: u8,
    pub talent: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type)]
pub struct CharacterState {
    pub level: Level,
    pub ascension: Ascension,
    pub eidolon: Eidolon,
    pub skills: CharacterSkillState,
    pub traces: CharacterTraceState,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type)]
pub struct LightConeState {
    pub level: Level,
    pub ascension: Ascension,
    // Superimposition should be 0-indexed (need to convert from 1-indexed when reading from scanner)
    pub superimposition: Superimposition,
}

pub fn promote<Spec>(promotions: &Promotions<Spec>, level: Level, ascension: Ascension, stat_fn: fn(&Spec) -> PromotionStepSpec) -> f64 {
    let spec = &promotions.values[ascension as usize];
    let spec = stat_fn(spec);

    return spec.base + (level as f64 - 1.0) * spec.step;
}

pub fn calculate_character_base_stats(character: (Character, CharacterState), light_cone: Option<(LightCone, LightConeState)>) -> CharacterStats {
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

        effect_hit_rate: 0.0,
    };

    if let Some(light_cone) = light_cone {
        let (light_cone, light_cone_state) = light_cone;

        let light_cone_promotions = use_light_cone_promotions(light_cone);

        stats.hp  += promote(&light_cone_promotions, light_cone_state.level, light_cone_state.ascension, |s| s.hp);
        stats.atk += promote(&light_cone_promotions, light_cone_state.level, light_cone_state.ascension, |s| s.atk);
        stats.def += promote(&light_cone_promotions, light_cone_state.level, light_cone_state.ascension, |s| s.def);
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
            ascension: 5 as Ascension,
            eidolon: 0 as Eidolon,
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
            },
        });
        let light_cone = Some((LightCone::IShallBeMyOwnSword, LightConeState {
            level: 56,
            ascension: 4,
            superimposition: 1,
        }));
        let stats = calculate_character_base_stats(character, light_cone);
        assert_float_relative_eq!(stats.hp,  1230.768 + 805.2);
        assert_float_relative_eq!(stats.atk, 582.12   + 402.6);
        assert_float_relative_eq!(stats.def, 415.80   + 274.5);
    }
}
