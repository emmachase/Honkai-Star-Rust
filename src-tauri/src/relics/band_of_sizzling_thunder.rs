use crate::data::Element;

use super::{RelicSetKit, RelicSetKitParams};

pub struct BandOfSizzlingThunder2Piece;
pub struct BandOfSizzlingThunder4Piece;
pub static BAND_OF_SIZZLING_THUNDER_2P: BandOfSizzlingThunder2Piece = BandOfSizzlingThunder2Piece;
pub static BAND_OF_SIZZLING_THUNDER_4P: BandOfSizzlingThunder4Piece = BandOfSizzlingThunder4Piece;

impl RelicSetKit for BandOfSizzlingThunder2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Thunder {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}

impl RelicSetKit for BandOfSizzlingThunder4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.band_of_sizzling_thunder_4p {
            p.boosts.atk_pct += 0.20;
        }
    }
}
