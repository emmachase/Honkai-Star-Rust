use super::{RelicSetKit, RelicSetKitParams};

pub struct ThiefOfShootingMeteor2Piece;
pub struct ThiefOfShootingMeteor4Piece;
pub static THIEF_OF_SHOOTING_METEOR_2P: ThiefOfShootingMeteor2Piece = ThiefOfShootingMeteor2Piece;
pub static THIEF_OF_SHOOTING_METEOR_4P: ThiefOfShootingMeteor4Piece = ThiefOfShootingMeteor4Piece;

impl RelicSetKit for ThiefOfShootingMeteor2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.break_effect += 0.16;
    }
}

impl RelicSetKit for ThiefOfShootingMeteor4Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.break_effect += 0.16;
    }
}
