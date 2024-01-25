use super::{RelicSetKit, RelicSetKitParams};

pub struct KnightOfPurityPalace2Piece;
pub struct KnightOfPurityPalace4Piece;
pub static KNIGHT_OF_PURITY_PALACE_2P: KnightOfPurityPalace2Piece = KnightOfPurityPalace2Piece;
pub static KNIGHT_OF_PURITY_PALACE_4P: KnightOfPurityPalace4Piece = KnightOfPurityPalace4Piece;

impl RelicSetKit for KnightOfPurityPalace2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.def_pct += 0.15;
    }
}

impl RelicSetKit for KnightOfPurityPalace4Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.shield_pct += 0.20;
    }
}
