use crate::{characters::StatColumnType, data::Element};

use super::{RelicSetKit, RelicSetKitParams};

pub struct HunterOfGlacialForest2Piece;
pub struct HunterOfGlacialForest4Piece;

impl RelicSetKit for HunterOfGlacialForest2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Ice {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }

    fn apply_common_conditionals(&self, _p: RelicSetKitParams) {
        // Nothing to do
    }

    fn apply_stat_type_conditionals(&self, _p: RelicSetKitParams, _stat_type: StatColumnType) {
        // Nothing to do
    }
}

impl RelicSetKit for HunterOfGlacialForest4Piece {
    fn apply_base_passives(&self, _p: RelicSetKitParams) {
        // Nothing to do
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.hunter_of_glacial_forest_4p {
            p.boosts.crit_dmg += 0.25;
        }
    }

    fn apply_stat_type_conditionals(&self, _p: RelicSetKitParams, _stat_type: StatColumnType) {
        // Nothing to do
    }
}

