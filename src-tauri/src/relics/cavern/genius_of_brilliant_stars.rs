use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct GeniusOfBrilliantStars2Piece;
pub struct GeniusOfBrilliantStars4Piece;
pub static GENIUS_OF_BRILLIANT_STARS_2P: GeniusOfBrilliantStars2Piece = GeniusOfBrilliantStars2Piece;
pub static GENIUS_OF_BRILLIANT_STARS_4P: GeniusOfBrilliantStars4Piece = GeniusOfBrilliantStars4Piece;

impl RelicSetKit for GeniusOfBrilliantStars2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Quantum {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}

impl RelicSetKit for GeniusOfBrilliantStars4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        p.boosts.def_shred += 0.10;

        if p.conditionals.genius_of_brilliant_stars_4p {
            p.boosts.def_shred += 0.10;
        }
    }
}
