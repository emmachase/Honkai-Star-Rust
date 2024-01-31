use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct HunterOfGlacialForest2Piece;
pub struct HunterOfGlacialForest4Piece;
pub static HUNTER_OF_GLACIAL_FOREST_2P: HunterOfGlacialForest2Piece = HunterOfGlacialForest2Piece;
pub static HUNTER_OF_GLACIAL_FOREST_4P: HunterOfGlacialForest4Piece = HunterOfGlacialForest4Piece;

impl RelicSetKit for HunterOfGlacialForest2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Ice {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}

impl RelicSetKit for HunterOfGlacialForest4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.hunter_of_glacial_forest_4p {
            p.boosts.crit_dmg += 0.25;
        }
    }
}
