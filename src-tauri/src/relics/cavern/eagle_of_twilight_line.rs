use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct EagleOfTwilightLine2Piece;
pub static EAGLE_OF_TWILIGHT_LINE_2P: EagleOfTwilightLine2Piece = EagleOfTwilightLine2Piece;

impl RelicSetKit for EagleOfTwilightLine2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.elemental_dmg_boost[Element::Wind] += 0.10;
    }
}
