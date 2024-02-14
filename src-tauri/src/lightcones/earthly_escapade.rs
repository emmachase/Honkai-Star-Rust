use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;
use specta::Type;

use crate::{promotions::LightConeState, damage::{Boosts, EnemyConfig}, data::use_light_cone_effects, data_mappings::LightCone, util::deserialize::deserialize_u8};

use super::LightConeKit;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
pub struct EarthlyEscapadeConfig {
    pub has_mask: bool,
}

pub struct EarthlyEscapade {
    pub descriptions: Vec<EarthlyEscapadeDesc>,
    pub config: EarthlyEscapadeConfig,
}

impl EarthlyEscapade {
    pub fn new(config: EarthlyEscapadeConfig) -> Self {
        return Self {
            descriptions: EarthlyEscapadeDesc::get(),
            config,
        }
    }
}

/**
 * Increases the wearer's CRIT DMG by #1[i]%. At the start of the battle, the wearer gains Mask,
 * lasting for #6[i] turn(s). While the wearer has Mask, the wearer's allies have their CRIT Rate
 * increased by #5[i]% and their CRIT DMG increased by #2[i]%. For every 1 Skill Point the wearer
 * recovers (including Skill Points that exceed the limit), they gain 1 stack of Radiant Flame.
 * And when the wearer has #4[i] stacks of Radiant Flame, all the stacks are removed, and they
 * gain Mask, lasting for #3[i] turn(s).
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
pub struct EarthlyEscapadeDesc {
    pub base_crit_dmg_pct: f64,
    pub mask_crit_dmg_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    pub earned_mask_duration: u8,
    #[serde(deserialize_with = "deserialize_u8")]
    pub radiant_flame_stacks: u8,
    pub mask_crit_rate_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    pub initial_mask_duration: u8,
}

impl EarthlyEscapadeDesc {
    pub fn get() -> Vec<Self> {
        return use_light_cone_effects(LightCone::EarthlyEscapade).superimpositions();
    }
}

impl LightConeKit for EarthlyEscapade {
    fn apply_base_passives(&self, _enemy_config: &EnemyConfig, light_cone_state: &LightConeState, boosts: &mut Boosts) {
        let desc = self.descriptions[light_cone_state.superimposition as usize];

        boosts.crit_dmg += desc.base_crit_dmg_pct;
    }

    fn apply_base_combat_passives(&self, _enemy_config: &EnemyConfig, light_cone_state: &LightConeState, boosts: &mut Boosts) {
        let desc = self.descriptions[light_cone_state.superimposition as usize];

        if self.config.has_mask {
            boosts.crit_dmg += desc.mask_crit_dmg_pct;
            boosts.crit_rate += desc.mask_crit_rate_pct;
        }
    }
}
