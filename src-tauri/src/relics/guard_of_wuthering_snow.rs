use super::{RelicSetKit, RelicSetKitParams};

pub struct GuardOfWutheringSnow2Piece;
pub static GUARD_OF_WUTHERING_SNOW_2P: GuardOfWutheringSnow2Piece = GuardOfWutheringSnow2Piece;

impl RelicSetKit for GuardOfWutheringSnow2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.dmg_reduction += 0.08;
    }
}
