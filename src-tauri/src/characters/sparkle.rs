use serde::Serialize;
use serde_tuple::Deserialize_tuple;
use specta::Type;

use crate::{col, damage::{calc_damage_multiplier, Boosts, CharacterStats, EnemyConfig}, data::{use_character, use_character_skill}, data_mappings::Character, promotions::CharacterState, shared_configs, util::deserialize::deserialize_u8, wrong_config};

use super::{CharacterKit, StatColumnDesc, StatColumnType};

shared_configs! {
    prefix Sparkle;

    Base { }
    Teammate { cd_stat: f64 }

    Shared {
        skill_cd_buff: bool,
        cipher_buff: bool,
        talent_dmg_stacks: u8,
        quantum_allies: u8
    }
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

impl Sparkle {
    fn apply_skill(&self, character_state: &CharacterState, config: SparkleConfig, boosts: &mut Boosts, crit_dmg: f64) {
        let config = SparkleSharedConfig::from(config);

        let skill = self.descriptions.skill[character_state.skills.skill as usize];

        // Skill CD Buff
        if config.skill_cd_buff {
            let mut pct = skill.crit_dmg_pct;
            if character_state.eidolon >= 6 {
                pct += 0.3;
            }

            boosts.crit_dmg += crit_dmg * pct + skill.crit_dmg_flat;
        }
    }
}

impl CharacterKit for Sparkle {
    fn apply_shared_combat_effects(&self, _enemy_config: &EnemyConfig, own_character_state: &CharacterState, boosts: &mut Boosts) {
        let config = SparkleSharedConfig::from(self.config);

        let talent = self.descriptions.talent[own_character_state.skills.talent as usize];
        let ultimate = self.descriptions.ultimate[own_character_state.skills.ult as usize];

        // Talent
        let mut dmg_boost_per_stack = talent.dmg_boost_pct;
        if config.cipher_buff {
            dmg_boost_per_stack += ultimate.dmg_boost_pct;
        }

        boosts.all_type_dmg_boost += dmg_boost_per_stack * config.talent_dmg_stacks as f64;
        if own_character_state.eidolon >= 2 {
            boosts.def_shred += 0.08 * config.talent_dmg_stacks as f64;
        }

        // "Nocturne" A6
        if own_character_state.traces.ability_3 {
            boosts.atk_pct += 0.15;
            boosts.atk_pct += match config.quantum_allies {
                1 => 0.5,
                2 => 0.15,
                3 => 0.30,
                _ => 0.0,
            };

            if own_character_state.eidolon >= 1 {
                boosts.atk_pct += 0.40;
            }
        }
    }

    fn apply_teammate_combat_effects(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, boosts: &mut Boosts) {
        let SparkleConfig::Teammate(config) = self.config else { wrong_config!(SparkleTeammateConfig) };
        self.apply_skill(character_state, self.config, boosts, config.cd_stat);
    }

    fn apply_general_conditionals(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, character_stats: &CharacterStats, boosts: &mut Boosts) {
        self.apply_skill(character_state, self.config, boosts, character_stats.crit_dmg(boosts));
    }

    fn get_stat_columns(&self, _enemy_config: &EnemyConfig) -> Vec<StatColumnDesc> {
        vec![
            col! { BasicDamage: [1.0] },
        ]
    }

    fn compute_stat_column(&self, column_type: super::StatColumnType, split: (usize, &f64), character_state: &CharacterState, character_stats: &CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64 {
        let damage_multiplier = calc_damage_multiplier(character_stats.element, character_stats, enemy_config, boosts);

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
