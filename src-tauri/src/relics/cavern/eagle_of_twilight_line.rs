use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct EagleOfTwilightLine2Piece;
pub static EAGLE_OF_TWILIGHT_LINE_2P: EagleOfTwilightLine2Piece = EagleOfTwilightLine2Piece;

impl RelicSetKit for EagleOfTwilightLine2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Wind {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}
