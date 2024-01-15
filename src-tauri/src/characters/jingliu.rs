use serde_tuple::Deserialize_tuple;

use crate::{damage::{Boosts, calculate_damage_with_avg_crits, EnemyConfig}, promotions::CharacterState, data::{use_character, use_character_skill}, data_mappings::Character, util::deserialize::deserialize_u8};

use super::common::{CharacterKit, StatColumnType};

pub struct Jingliu {
    pub descriptions: JingliuDescriptions,

    pub enhanced_state: bool, // Spectral Transmigration
    pub hp_drain_pct: f64, // 0-1, how much of ATK% cap is available
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
struct BasicDesc {
    atk_pct: f64,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy and obtains #2[i] stack(s) of Syzygy. 
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
struct NormalSkillDesc {
    atk_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    syzygy_stacks: u8,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal to #3[i]% 
 * of Jingliu's ATK to any adjacent enemies. Gains #2[i] stack(s) of Syzygy after attack ends. 
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
struct UltimateDesc {
    atk_pct_main: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    syzygy_stacks: u8,
    atk_pct_adj: f64,
    unknown: f64,
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
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
struct TalentDesc {
    unknown: f64,
    consume_hp_pct: f64,
    atk_pct_from_hp: f64,
    atk_pct_cap: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    required_stacks: u8,
    action_advance_pct: f64,
    crit_rate_pct: f64,
}

/**
 * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal 
 * to #3[i]% of Jingliu's ATK to adjacent enemies. Consumes #2[i] stack(s) of Syzygy. Using 
 * this ability does not consume Skill Points.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
struct EnhancedSkillDesc {
    atk_pct_main: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    syzygy_stacks: u8,
    atk_pct_adj: f64,
}

pub struct JingliuDescriptions {
    basic: Vec<BasicDesc>,
    normal_skill: Vec<NormalSkillDesc>,
    ultimate: Vec<UltimateDesc>,
    talent: Vec<TalentDesc>,
    enhanced_skill: Vec<EnhancedSkillDesc>,
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
    fn apply_static_passives(&self, _enemy_config: &EnemyConfig, character_state: &CharacterState, boosts: &mut Boosts) {
        if self.enhanced_state {
            if character_state.traces.ability_1 {
                boosts.effect_res += 0.35; // TODO: Do i care about getting this from the desc?
            }

            let talent = self.descriptions.talent[character_state.skills.talent as usize];

            boosts.crit_rate += talent.crit_rate_pct;
            boosts.atk_pct += talent.atk_pct_cap * self.hp_drain_pct;
        }
    }

    fn apply_conditional_passives(&self, _enemy_config: &EnemyConfig, stat_type: StatColumnType, character_state: &CharacterState, boosts: &mut Boosts) {
        if stat_type == StatColumnType::UltimateDamage {
            if self.enhanced_state && character_state.traces.ability_3 {
                boosts.all_type_dmg_boost += 0.2;
            }
        }
    }

    fn get_stat_columns(&self) -> Vec<StatColumnType> {
        return vec![
            StatColumnType::BasicDamage,
            StatColumnType::SkillDamage,
            StatColumnType::UltimateDamage
        ]
    }

    fn compute_stat_column(&self, column_type: StatColumnType, character_state: &CharacterState, character_stats: &crate::damage::CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64 {
        match column_type {
            StatColumnType::BasicDamage => {
                if self.enhanced_state {
                    return 0.0; // Basic is not available in enhanced state
                }

                let desc = self.descriptions.basic[character_state.skills.basic as usize];
                let base_dmg = desc.atk_pct * character_stats.atk(boosts);
                return calculate_damage_with_avg_crits(base_dmg, character_stats, enemy_config, boosts);
            },
            StatColumnType::SkillDamage => {
                let atk = character_stats.atk(boosts);

                if self.enhanced_state {
                    let desc = self.descriptions.enhanced_skill[character_state.skills.skill as usize];

                    let base_main_dmg = desc.atk_pct_main * atk;
                    let main_dmg = calculate_damage_with_avg_crits(base_main_dmg, character_stats, enemy_config, boosts);
                    
                    let base_adj_dmg = desc.atk_pct_adj * atk;
                    let adj_dmg = calculate_damage_with_avg_crits(base_adj_dmg, character_stats, enemy_config, boosts);

                    return main_dmg + adj_dmg * (enemy_config.count - 1).min(2) as f64;
                } else {
                    let desc = self.descriptions.normal_skill[character_state.skills.skill as usize];
                    let base_dmg = desc.atk_pct * atk;
                    return calculate_damage_with_avg_crits(base_dmg, character_stats, enemy_config, boosts);
                }
            },
            StatColumnType::UltimateDamage => {
                let atk = character_stats.atk(&boosts);

                let desc = self.descriptions.ultimate[character_state.skills.ult as usize];
                let base_main_dmg = desc.atk_pct_main * atk;
                let main_dmg = calculate_damage_with_avg_crits(base_main_dmg, character_stats, enemy_config, &boosts);
                
                let base_adj_dmg = desc.atk_pct_adj * atk;
                let adj_dmg = calculate_damage_with_avg_crits(base_adj_dmg, character_stats, enemy_config, &boosts);

                return main_dmg + adj_dmg * (enemy_config.count - 1).min(2) as f64;
            },
            _ => panic!("Invalid stat column type for Jingliu: {:?}", column_type),
        }
    }
}
