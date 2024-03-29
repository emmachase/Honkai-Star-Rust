use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct FiresmithOfLavaForging2Piece;
pub struct FiresmithOfLavaForging4Piece;
pub static FIRESMITH_OF_LAVA_FORGING_2P: FiresmithOfLavaForging2Piece = FiresmithOfLavaForging2Piece;
pub static FIRESMITH_OF_LAVA_FORGING_4P: FiresmithOfLavaForging4Piece = FiresmithOfLavaForging4Piece;

impl RelicSetKit for FiresmithOfLavaForging2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.elemental_dmg_boost[Element::Fire] += 0.10;
    }
}

impl RelicSetKit for FiresmithOfLavaForging4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.firesmith_of_lava_forging_4p {
            p.boosts.elemental_dmg_boost[Element::Fire] += 0.12;
        }
    }
}
