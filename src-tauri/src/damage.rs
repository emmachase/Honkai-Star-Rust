use crate::data::Element;

pub type Level = u8;
pub type Ascension = u8;
pub type Eidolon = u8;
pub type Superimposition = u8;

#[derive(Debug, Clone, Copy)]
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
    pub elemental_dmg_bonus: f64,
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
        return self.spd + boosts.spd;
    }

    pub fn effect_res(&self, boosts: &Boosts) -> f64 {
        return self.effect_res + boosts.effect_res;
    }

    pub fn crit_rate(&self, boosts: &Boosts) -> f64 {
        return self.crit_rate + boosts.crit_rate;
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
}

#[derive(Debug, Clone, Copy)]
pub struct EnemyConfig {
    pub count: u8,
    pub level: Level,

    pub resistance: f64,
    pub elemental_weakness: bool,
    pub weakness_broken: bool,
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
    
    pub spd: f64,
    pub effect_res: f64,
    pub effect_hit_rate: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub break_effect: f64,
    pub energy_recharge: f64,
    pub outgoing_healing_boost: f64,
    pub elemental_dmg_boost: f64,

    pub all_type_dmg_boost: f64,
    pub extra_vulnerability: f64,
    pub def_shred: f64, // enemy def_reduction + attacker def_ignore
    pub res_pen: f64,
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
        damage_boost + character_stats.elemental_dmg_bonus + boosts.elemental_dmg_boost
    } else {
        damage_boost // No bonus damage if enemy is not weak to the element
    };

    return damage_boost * def_multiplier * res_multiplier * vulnerability_multiplier * broken_multiplier;
}

/**
 * Calculates the damage of a attack ignoring crits (DOT, or normal attack without crits)
 */
pub fn calculate_damage_common(
    base_dmg: f64,

    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    return base_dmg * calculate_common_multiplier(character_stats, enemy_config, boosts);
}

/**
 * Calculates the damage of a normal attack (Not DOT)
 * Called once for each type of attack (basic, skill, ult)
 */
pub fn calculate_damage_with_avg_crits(
    base_dmg: f64,

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
    let boosted_crit_rate = (character_stats.crit_rate + boosts.crit_rate).min(1.0);
    let boosted_crit_dmg = character_stats.crit_dmg + boosts.crit_dmg;
    let crit_multiplier = 1.0 + boosted_crit_rate * boosted_crit_dmg;

    let common_multiplier = calculate_common_multiplier(character_stats, enemy_config, boosts);

    return base_dmg * common_multiplier * crit_multiplier;
}

/**
 * Calculates the maximum damage of a normal attack (EQ: CR=1) (Not DOT)
 */
pub fn calculate_damage_with_max_crits(
    base_dmg: f64,

    character_stats: &CharacterStats,
    enemy_config: &EnemyConfig,
    boosts: &Boosts,
) -> f64 {
    let boosted_crit_dmg = (character_stats.crit_dmg  + boosts.crit_dmg ).min(1.0);
    let crit_multiplier = 1.0 + boosted_crit_dmg;

    let common_multiplier = calculate_common_multiplier(character_stats, enemy_config, boosts);

    return base_dmg * common_multiplier * crit_multiplier;
}

