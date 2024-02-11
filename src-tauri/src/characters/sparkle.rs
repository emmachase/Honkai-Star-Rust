use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;
use specta::Type;

use crate::{col, damage::{calc_damage_multiplier, Boosts, CharacterStats, EnemyConfig}, data::{use_character, use_character_skill}, data_mappings::Character, promotions::CharacterState, util::deserialize::deserialize_u8};

use super::{CharacterKit, StatColumnDesc, StatColumnType};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
pub struct SparkleConfig {
    pub skill_cd_buff: bool,
    pub cipher_buff: bool,
    pub talent_dmg_stacks: u8,
    pub quantum_allies: u8,
}

pub struct Sparkle {
    pub descriptions: SparkleDescriptions,
    pub config: SparkleConfig,
}

impl Sparkle {
    pub fn new(config: SparkleConfig) -> Self {
        return Self {
            descriptions: SparkleDescriptions::get(),
            config,
        }
    }
}

/**
 * Deals Quantum DMG equal to #1[i]% of Sparkle's ATK to a single enemy.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct SparkleBasicDesc {
    atk_pct: f64,
}

/**
 * Increases the CRIT DMG of a single target ally by #1[f1]% of Sparkle's
 * CRIT DMG plus #2[f1]%, lasting for #3[i] turn(s). And at the same time,
 * Advances Forward this ally's action by #4[i]%.\nWhen Sparkle uses this
 * ability on herself, the Action Advance effect will not trigger.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct SparkleSkillDesc {
    crit_dmg_pct: f64,
    crit_dmg_flat: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    duration: u8,
    action_advance: f64,
}

/**
 * Recovers #2[i] Skill Points for the team and grants all allies Cipher.
 * For allies with Cipher, each stack of the DMG Boost effect provided by
 * Sparkle's Talent additionally increases by #3[f1]%, lasting for #4[i]
 * turns.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct SparkleUltimateDesc {
    #[serde(deserialize_with = "deserialize_u8")]
    _unknown: u8,
    #[serde(deserialize_with = "deserialize_u8")]
    skill_points: u8,
    dmg_boost_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    duration: u8,
}

/**
 * While Sparkle is on the battlefield, additionally increases the max number
 * of Skill Points by #3[i]. Whenever an ally consumes 1 Skill Point, all
 * allies' DMG dealt increases by #2[f1]%. This effect lasts for #1[i] turn(s)
 * and can stack up to #4[i] time(s).
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct SparkleTalentDesc {
    #[serde(deserialize_with = "deserialize_u8")]
    duration: u8,
    dmg_boost_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    skill_points: u8,
    #[serde(deserialize_with = "deserialize_u8")]
    stacks: u8,
}

#[derive(Debug, Serialize, Type)]
pub struct SparkleDescriptions {
    basic: Vec<SparkleBasicDesc>,
    skill: Vec<SparkleSkillDesc>,
    ultimate: Vec<SparkleUltimateDesc>,
    talent: Vec<SparkleTalentDesc>,
}

impl SparkleDescriptions {
    pub fn get() -> Self {
        let character = use_character(Character::Sparkle);

        return Self {
            basic: use_character_skill(&character.skills[0]).levels(),
            skill: use_character_skill(&character.skills[1]).levels(),
            ultimate: use_character_skill(&character.skills[2]).levels(),
            talent: use_character_skill(&character.skills[3]).levels(),
            // maze_desc: use_character_skill(&character.skills[4]).levels(),
            // technique_desc: use_character_skill(&character.skills[5]).levels(),
        };
    }
}

impl CharacterKit for Sparkle {
    fn apply_base_combat_passives(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, boosts: &mut Boosts) {
        let talent = self.descriptions.talent[character_state.skills.talent as usize];
        let ultimate = self.descriptions.ultimate[character_state.skills.ult as usize];

        // Talent
        let mut dmg_boost_per_stack = talent.dmg_boost_pct;
        if self.config.cipher_buff {
            dmg_boost_per_stack += ultimate.dmg_boost_pct;
        }

        boosts.all_type_dmg_boost += dmg_boost_per_stack * self.config.talent_dmg_stacks as f64;
        if character_state.eidolon >= 2 {
            boosts.def_shred += 0.08 * self.config.talent_dmg_stacks as f64;
        }

        // "Nocturne" A6
        if character_state.traces.ability_3 {
            boosts.atk_pct += 0.15;
            boosts.atk_pct += match self.config.quantum_allies {
                1 => 0.5,
                2 => 0.15,
                3 => 0.30,
                _ => 0.0,
            };

            if character_state.eidolon >= 1 {
                boosts.atk_pct += 0.40;
            }
        }
    }

    fn apply_common_conditionals(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, character_stats: &CharacterStats, boosts: &mut Boosts) {
        let skill = self.descriptions.skill[character_state.skills.skill as usize];

        // Skill CD Buff
        if self.config.skill_cd_buff {
            let mut pct = skill.crit_dmg_pct;
            if character_state.eidolon >= 6 {
                pct += 0.3;
            }

            boosts.crit_dmg += character_stats.crit_dmg(boosts) * pct + skill.crit_dmg_flat;
        }
    }

    fn get_stat_columns(&self, _enemy_config: &EnemyConfig) -> Vec<StatColumnDesc> {
        vec![
            col! { BasicDamage: [1.0] },
        ]
    }

    fn compute_stat_column(&self, column_type: super::StatColumnType, split: (usize, &f64), character_state: &CharacterState, character_stats: &CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64 {
        let damage_multiplier = calc_damage_multiplier(character_stats, enemy_config, boosts);

        return split.1 * match column_type {
            StatColumnType::BasicDamage => {
                let desc = self.descriptions.basic[character_state.skills.basic as usize];
                let base_dmg = desc.atk_pct * character_stats.atk(boosts);

                base_dmg * damage_multiplier
            },
            _ => panic!("Invalid column type for Sparkle"),
        }
    }
}
