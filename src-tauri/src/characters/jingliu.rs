use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;
use specta::Type;

use crate::{damage::{Boosts, calc_damage_multiplier, EnemyConfig, CharacterStats}, promotions::CharacterState, data::{use_character, use_character_skill}, data_mappings::Character, util::deserialize::deserialize_u8, col};

use super::{CharacterKit, StatColumnType, StatColumnDesc};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
pub struct JingliuConfig {
    pub enhanced_state: bool, // Spectral Transmigration
    pub hp_drain_pct: f64, // 0-1, how much of ATK% cap is available

    pub e1_crit_dmg: bool,
    pub e2_skill_buff: bool,
}

pub struct Jingliu {
    pub descriptions: JingliuDescriptions,
    pub config: JingliuConfig,
}

impl Jingliu {
    pub fn new(config: JingliuConfig) -> Self {
        return Self {
            descriptions: JingliuDescriptions::get(),
            config,
        }
    }
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct JingliuBasicDesc {
    atk_pct: f64,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy and obtains #2[i] stack(s) of Syzygy.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct JingliuNormalSkillDesc {
    atk_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    _syzygy_stacks: u8,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal to #3[i]%
 * of Jingliu's ATK to any adjacent enemies. Gains #2[i] stack(s) of Syzygy after attack ends.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct JingliuUltimateDesc {
    atk_pct_main: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    _syzygy_stacks: u8,
    _atk_pct_adj: f64,
    _unknown: f64,
}

/**
 * When Jingliu has #5[i] stack(s) of Syzygy, she enters the Spectral Transmigration state with her
 * Action Advanced by #6[i]% and her CRIT Rate increases by #7[i]%. Then, Jingliu's Skill
 * \"Transcendent Flash\" is enhanced to \"Moon On Glacial River,\" and only this enhanced Skill is
 * available for use in battle. When Jingliu uses an attack in the Spectral Transmigration state,
 * she consumes HP from all other allies equal to #2[i]% of their respective Max HP (this cannot
 * reduce allies' HP to lower than 1). Jingliu's ATK increases by #3[i]% of the total HP consumed
 * from all allies in this attack, capped at #4[i]% of her base ATK, lasting until the current attack
 * ends. Jingliu cannot enter the Spectral Transmigration state again until the current
 * Spectral Transmigration state ends. Syzygy can stack up to 3 times. When Syzygy stacks become 0,
 * Jingliu will exit the Spectral Transmigration state.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct JingliuTalentDesc {
    _unknown: f64,
    _consume_hp_pct: f64,
    _atk_pct_from_hp: f64,
    atk_pct_cap: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    _required_stacks: u8,
    _action_advance_pct: f64,
    crit_rate_pct: f64,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal
 * to #3[i]% of Jingliu's ATK to adjacent enemies. Consumes #2[i] stack(s) of Syzygy. Using
 * this ability does not consume Skill Points.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple, Serialize, Type)]
struct JingliuEnhancedSkillDesc {
    atk_pct_main: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    _syzygy_stacks: u8,
    _atk_pct_adj: f64,
}

#[derive(Debug, Serialize, Type)]
pub struct JingliuDescriptions {
    basic: Vec<JingliuBasicDesc>,
    normal_skill: Vec<JingliuNormalSkillDesc>,
    ultimate: Vec<JingliuUltimateDesc>,
    talent: Vec<JingliuTalentDesc>,
    enhanced_skill: Vec<JingliuEnhancedSkillDesc>,
}

impl JingliuDescriptions {
    pub fn get() -> Self {
        let character = use_character(Character::Jingliu);

        return Self {
            basic: use_character_skill(&character.skills[0]).levels(),
            normal_skill: use_character_skill(&character.skills[1]).levels(),
            ultimate: use_character_skill(&character.skills[2]).levels(),
            talent: use_character_skill(&character.skills[3]).levels(),
            // maze_desc: use_character_skill(&character.skills[4]).levels(),
            // technique_desc: use_character_skill(&character.skills[5]).levels(),
            enhanced_skill: use_character_skill(&character.skills[6]).levels(),
        }
    }
}

// TODO: get descs from type instead of order

impl CharacterKit for Jingliu {
    fn apply_base_combat_effects(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, boosts: &mut Boosts) {
        if self.config.enhanced_state {
            if character_state.traces.ability_1 {
                boosts.effect_res += 0.35;
            }

            let talent = self.descriptions.talent[character_state.skills.talent as usize];

            let mut atk_pct_cap = talent.atk_pct_cap;
            if character_state.eidolon >= 4 {
                atk_pct_cap += 0.3;
            }

            boosts.crit_rate += talent.crit_rate_pct;
            boosts.atk_pct += atk_pct_cap * self.config.hp_drain_pct;

            if character_state.eidolon >= 6 {
                boosts.crit_dmg += 0.5;
            }
        }

        if character_state.eidolon >= 1 && self.config.e1_crit_dmg {
            boosts.crit_dmg += 0.24;
        }
    }

    fn apply_stat_type_conditionals(&self, _enemy_config: &EnemyConfig, stat_type: StatColumnType, character_state: &CharacterState, _character_stats: &CharacterStats, boosts: &mut Boosts) {
        match stat_type {
            StatColumnType::SkillDamage => {
                if self.config.enhanced_state && character_state.eidolon >= 2 && self.config.e2_skill_buff {
                    boosts.all_type_dmg_boost += 0.8;
                }
            }

            StatColumnType::UltimateDamage => {
                if self.config.enhanced_state && character_state.traces.ability_3 {
                    boosts.all_type_dmg_boost += 0.2;
                }
            },

            _ => {},
        }
    }

    fn get_stat_columns(&self, _enemy_config: &EnemyConfig) -> Vec<StatColumnDesc> {
        return if self.config.enhanced_state {
            vec![
                col! { SkillDamage: [0.1, 0.1, 0.1, 0.2, 0.5] },
                col! { UltimateDamage: [1.0] },
            ]
        } else {
            vec![
                col! { BasicDamage: [0.3, 0.7] },
                col! { SkillDamage: [0.1, 0.1, 0.1, 0.2, 0.5] },
                col! { UltimateDamage: [1.0] },
            ]
        }
    }

    fn compute_stat_column(&self, column_type: StatColumnType, (_, split): (usize, &f64), character_state: &CharacterState, character_stats: &CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64 {
        let damage_multiplier = calc_damage_multiplier(character_stats.element, character_stats, enemy_config, boosts);

        return split * match column_type {
            StatColumnType::BasicDamage => {
                if self.config.enhanced_state {
                    panic!("Basic is not available in enhanced state")
                }

                let desc = self.descriptions.basic[character_state.skills.basic as usize];
                let base_dmg = desc.atk_pct * character_stats.atk(boosts);

                base_dmg * damage_multiplier
            },
            StatColumnType::SkillDamage => {
                let atk = character_stats.atk(boosts);

                if self.config.enhanced_state {
                    let desc = self.descriptions.enhanced_skill[character_state.skills.skill as usize];

                    let mut base_main_dmg = desc.atk_pct_main * atk;
                    if character_state.eidolon >= 1 && enemy_config.count == 1 {
                        base_main_dmg += atk;
                    }

                    let main_dmg = base_main_dmg * damage_multiplier;

                    // let base_adj_dmg = desc.atk_pct_adj * atk;
                    // let adj_dmg = base_adj_dmg * damage_multiplier;

                    main_dmg // + adj_dmg * (enemy_config.count - 1).min(2) as f64;
                } else {
                    let desc = self.descriptions.normal_skill[character_state.skills.skill as usize];
                    let base_dmg = desc.atk_pct * atk;

                    base_dmg * damage_multiplier
                }
            },
            StatColumnType::UltimateDamage => {
                let atk = character_stats.atk(&boosts);

                let desc = self.descriptions.ultimate[character_state.skills.ult as usize];

                let mut base_main_dmg = desc.atk_pct_main * atk;
                if character_state.eidolon >= 1 && enemy_config.count == 1 {
                    base_main_dmg += atk;
                }

                let main_dmg = base_main_dmg * damage_multiplier;

                // let base_adj_dmg = desc.atk_pct_adj * atk;
                // let adj_dmg = base_adj_dmg * damage_multiplier;

                main_dmg // + adj_dmg * (enemy_config.count - 1).min(2) as f64;
            },
            _ => panic!("Invalid stat column type for Jingliu: {:?}", column_type),
        }
    }
}
