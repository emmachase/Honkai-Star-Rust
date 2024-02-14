pub mod i_shall_be_my_own_sword;
pub mod earthly_escapade;

use serde::{Serialize, Deserialize};
use specta::Type;

use crate::{promotions::LightConeState, damage::{Boosts, EnemyConfig}, characters::StatColumnType, data_mappings::LightCone};

pub trait LightConeKit {
    /**
     * This function is called once outside of the permutation loop.
     * It should apply light-cone passive effects that affect the character's base stats. (i.e. it shows up in the character's stat sheet)
     */
    fn apply_base_passives(&self, _enemy_config: &EnemyConfig, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {}

    /**
     * This function is called once outside of the permutation loop.
     * It should apply light-cone passive effects that affect the character's combat stats. (i.e. it only shows up during combat)
     */
    fn apply_base_combat_passives(&self, _enemy_config: &EnemyConfig, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {}

    /**
     * This function is called once for each relic permutation.
     * It should apply light-cone effects that are conditional based on relic stats (e.g. +10% DMG when SPD > 160)
     * If the effect does not depend on relic stats, it should be applied in [`LightConeKit::apply_base_combat_passives()`] instead.
     */
    fn apply_common_conditionals(&self, _enemy_config: &EnemyConfig, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {}

    /**
     * This function is called multiple times for each relic permutation.
     * It should apply light-cone effects that are conditional based on the type of stat being calculated (e.g. +10% Ultimate DMG)
     */
    fn apply_stat_type_conditionals(&self, _enemy_config: &EnemyConfig, _stat_type: StatColumnType, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {}
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub enum LightConeConfig {
    IShallBeMyOwnSword(i_shall_be_my_own_sword::IShallBeMyOwnSwordConfig),
    EarthlyEscapade(earthly_escapade::EarthlyEscapadeConfig),
}

impl LightConeConfig {
    pub fn get_light_cone_id(&self) -> LightCone {
        match self {
            LightConeConfig::IShallBeMyOwnSword(_) => LightCone::IShallBeMyOwnSword,
            LightConeConfig::EarthlyEscapade(_) => LightCone::EarthlyEscapade,
        }
    }

    pub fn get_kit(&self) -> Box<dyn LightConeKit+Send+Sync> {
        match self {
            LightConeConfig::IShallBeMyOwnSword(config) => Box::new(i_shall_be_my_own_sword::IShallBeMyOwnSword::new(*config)),
            LightConeConfig::EarthlyEscapade(config) => Box::new(earthly_escapade::EarthlyEscapade::new(*config)),
        }
    }
}
