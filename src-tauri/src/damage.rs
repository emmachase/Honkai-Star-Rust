use std::ops::{Add, Index, IndexMut};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::data::Element;

pub type Level = u8;
pub type Ascension = u8;
pub type Eidolon = u8;
pub type Superimposition = u8;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type, Default)]
#[serde(transparent)]
pub struct ElementalDmgBoost([f64; Element::COUNT]);

impl Index<Element> for ElementalDmgBoost {
    type Output = f64;

    fn index(&self, index: Element) -> &Self::Output {
        return &self.0[index as usize];
    }
}

impl IndexMut<Element> for ElementalDmgBoost {
    fn index_mut(&mut self, index: Element) -> &mut Self::Output {
        return &mut self.0[index as usize];
    }
}

impl Add for ElementalDmgBoost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.0;
        for i in 0..Element::COUNT {
            result[i] += rhs.0[i];
        }
        return Self(result);
    }
}

impl Add<f64> for ElementalDmgBoost {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let mut result = self.0;
        for i in 0..Element::COUNT {
            result[i] += rhs;
        }
        return Self(result);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type)]
pub struct CharacterStats {
    pub level: Level,
    pub ascension: Ascension,
    pub element: Element,

    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub spd: f64,
    pub effect_res: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub break_effect: f64,
    pub energy_recharge: f64,
    pub outgoing_healing_boost: f64,
    // pub elemental_dmg_bonus: f64,
    pub elemental_dmg_boost: ElementalDmgBoost,

    // Boost-only, included for convenience
    pub effect_hit_rate: f64,
}

impl CharacterStats {
    pub fn hp(&self, boosts: &Boosts) -> f64 {
        return self.hp * (1.0 + boosts.hp_pct) + boosts.hp_flat;
    }

    pub fn atk(&self, boosts: &Boosts) -> f64 {
        return self.atk * (1.0 + boosts.atk_pct) + boosts.atk_flat;
    }

    pub fn def(&self, boosts: &Boosts) -> f64 {
        return self.def * (1.0 + boosts.def_pct) + boosts.def_flat;
    }

    pub fn spd(&self, boosts: &Boosts) -> f64 {
        return self.spd * (1.0 + boosts.spd_pct) + boosts.spd_flat;
    }

    pub fn effect_res(&self, boosts: &Boosts) -> f64 {
        return self.effect_res + boosts.effect_res;
    }

    pub fn crit_rate(&self, boosts: &Boosts) -> f64 {
        return (self.crit_rate + boosts.crit_rate).min(1.0);
    }

    pub fn crit_dmg(&self, boosts: &Boosts) -> f64 {
        return self.crit_dmg + boosts.crit_dmg;
    }

    pub fn break_effect(&self, boosts: &Boosts) -> f64 {
        return self.break_effect + boosts.break_effect;
    }

    pub fn energy_recharge(&self, boosts: &Boosts) -> f64 {
        return self.energy_recharge + boosts.energy_recharge;
    }

    pub fn outgoing_healing_boost(&self, boosts: &Boosts) -> f64 {
        return self.outgoing_healing_boost + boosts.outgoing_healing_boost;
    }

    pub fn elemental_dmg_bonus(&self, boosts: &Boosts) -> ElementalDmgBoost {
        return self.elemental_dmg_boost + boosts.elemental_dmg_boost;
    }

    pub fn effect_hit_rate(&self, boosts: &Boosts) -> f64 {
        return self.effect_hit_rate + boosts.effect_hit_rate;
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
pub struct EnemyConfig {
    pub count: u8,
    pub level: Level,

    pub resistance: f64,
    pub elemental_weakness: bool,
    pub weakness_broken: bool,
    pub debuff_count: u8,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        return Self {
            count: 3,
            level: 80,
            resistance: 0.2,
            elemental_weakness: true,
            weakness_broken: false,
            debuff_count: 3,
        };
    }
}

/**
 * Should include all boosts applied
 * - Character & Light Cone passives and conditionals
 * - Conditional set effects
 * - Extra boosts (team buffs)
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct Boosts {
    pub hp_flat: f64,
    pub hp_pct: f64,
    pub atk_flat: f64,
    pub atk_pct: f64,
    pub def_flat: f64,
    pub def_pct: f64,
    pub spd_flat: f64,
    pub spd_pct: f64,

    pub effect_res: f64,
    pub effect_hit_rate: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub break_effect: f64,
    pub energy_recharge: f64,
    pub outgoing_healing_boost: f64,
    pub elemental_dmg_boost: ElementalDmgBoost,

    pub all_type_dmg_boost: f64,
    pub extra_vulnerability: f64,
    pub def_shred: f64, // enemy def_reduction + attacker def_ignore
    pub res_pen: f64,

    pub shield_pct: f64,
    pub dmg_reduction: f64,
}

impl Add for Boosts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            hp_flat: self.hp_flat + rhs.hp_flat,
            hp_pct: self.hp_pct + rhs.hp_pct,
            atk_flat: self.atk_flat + rhs.atk_flat,
            atk_pct: self.atk_pct + rhs.atk_pct,
            def_flat: self.def_flat + rhs.def_flat,
            def_pct: self.def_pct + rhs.def_pct,

            spd_flat: self.spd_flat + rhs.spd_flat,
            spd_pct: self.spd_pct + rhs.spd_pct,

            effect_res: self.effect_res + rhs.effect_res,
            effect_hit_rate: self.effect_hit_rate + rhs.effect_hit_rate,
            crit_rate: self.crit_rate + rhs.crit_rate,
            crit_dmg: self.crit_dmg + rhs.crit_dmg,
            break_effect: self.break_effect + rhs.break_effect,
            energy_recharge: self.energy_recharge + rhs.energy_recharge,
            outgoing_healing_boost: self.outgoing_healing_boost + rhs.outgoing_healing_boost,
            elemental_dmg_boost: self.elemental_dmg_boost + rhs.elemental_dmg_boost,

            all_type_dmg_boost: self.all_type_dmg_boost + rhs.all_type_dmg_boost,
            extra_vulnerability: self.extra_vulnerability + rhs.extra_vulnerability,
            def_shred: self.def_shred + rhs.def_shred,
            res_pen: self.res_pen + rhs.res_pen,

            shield_pct: self.shield_pct + rhs.shield_pct,
            dmg_reduction: self.dmg_reduction + rhs.dmg_reduction,
        }
    }
}

impl Add<Boosts> for CharacterStats {
    type Output = CharacterStats;

    fn add(self, rhs: Boosts) -> Self::Output {
        return Self::Output {
            level: self.level,
            ascension: self.ascension,
            element: self.element,

            hp: self.hp(&rhs),
            atk: self.atk(&rhs),
            def: self.def(&rhs),
            spd: self.spd(&rhs),
            effect_res: self.effect_res(&rhs),
            crit_rate: self.crit_rate(&rhs),
            crit_dmg: self.crit_dmg(&rhs),
            break_effect: self.break_effect(&rhs),
            energy_recharge: self.energy_recharge(&rhs),
            outgoing_healing_boost: self.outgoing_healing_boost(&rhs),
            elemental_dmg_boost: self.elemental_dmg_bonus(&rhs) + rhs.all_type_dmg_boost, // TODO: Should we include all_type_dmg_boost here?

            effect_hit_rate: self.effect_hit_rate(&rhs),
        }
    }
}

fn calculate_def_multiplier(
    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    let adj_character_level: f64 = character_stats.level as f64 + 20.0;
    let adj_enemy_level    : f64 = enemy_config.level    as f64 + 20.0;

    let def_reduction = (1.0 - boosts.def_shred).max(0.0);

    return adj_character_level / (adj_enemy_level * def_reduction + adj_character_level);
}

/**
 * Common Multipliers between Normal atacks and Follow-up attacks
 * - DMG Boost
 * - DEF Multiplier
 * - RES Multiplier
 * - Vulnerability Multiplier
 */
fn calculate_common_multiplier(
    damage_element: Element,
    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    let def_multiplier = calculate_def_multiplier(character_stats, enemy_config, boosts);
    let broken_multiplier = if enemy_config.weakness_broken { 1.0 } else { 0.9 };

    let resistance = if enemy_config.elemental_weakness { 0.0 } else { enemy_config.resistance };
    let res_multiplier           = 1.0 - (resistance - boosts.res_pen); // Enemy RES can be negative, funky :)
    let vulnerability_multiplier = 1.0 + boosts.extra_vulnerability;
    let damage_boost             = 1.0 + boosts.all_type_dmg_boost;
    let damage_boost = if enemy_config.elemental_weakness {
        damage_boost + character_stats.elemental_dmg_boost[damage_element] + boosts.elemental_dmg_boost[damage_element]
    } else {
        damage_boost // No bonus damage if enemy is not weak to the element
    };

    return damage_boost * def_multiplier * res_multiplier * vulnerability_multiplier * broken_multiplier;
}

// /**
//  * Calculates the damage of a attack ignoring crits (DOT, or normal attack without crits)
//  */
// pub fn calculate_damage_common(
//     base_dmg: f64,

//     character_stats: &CharacterStats,
//     enemy_config: &EnemyConfig,
//     boosts: &Boosts,
// ) -> f64 {
//     return base_dmg * calculate_common_multiplier(character_stats, enemy_config, boosts);
// }

/**
 * Calculates the damage multiplier of a normal attack (Not DOT)
 * Crit DMG is applied according to average value based on crit rate.
 * Should be called once for each type of attack (basic, skill, ult)
 */
pub fn calc_damage_multiplier(
    damage_element: Element,
    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    /* Derivation for average damage taking into account crit rate:
     *  x = base damage
     *  min = x
     *  max = x + x * CD
     *  avg = (max * CR) + (min * (1 - CR))
     *  avg = ((x + x*CD) * CR) + (x * (1 - CR))
     *  avg = (x * (1 + CD) * CR) + (x * (1 - CR))
     *  avg = x * ((1 + CD) * CR + (1 - CR))
     *  avg = x * (CR + CR*CD + 1 - CR)
     *  avg = x * (1 + CR*CD)
     */
    let boosted_crit_rate = character_stats.crit_rate(boosts);
    let boosted_crit_dmg = character_stats.crit_dmg(boosts);
    let crit_multiplier = 1.0 + boosted_crit_rate * boosted_crit_dmg;

    let common_multiplier = calculate_common_multiplier(damage_element, character_stats, enemy_config, boosts);

    return common_multiplier * crit_multiplier;
}

/**
 * Calculates the maximum damage multiplier of a normal attack (EQ: CR=1) (Not DOT)
 */
pub fn _calc_max_damage_multiplier(
    damage_element: Element,
    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    let boosted_crit_dmg = (character_stats.crit_dmg  + boosts.crit_dmg).min(1.0);
    let crit_multiplier = 1.0 + boosted_crit_dmg;

    let common_multiplier = calculate_common_multiplier(damage_element, character_stats, enemy_config, boosts);

    return common_multiplier * crit_multiplier;
}
