use super::{RelicSetKit, RelicSetKitParams};

pub struct MessengerTraversingHackerspace2Piece;
pub struct MessengerTraversingHackerspace4Piece;
pub static MESSENGER_TRAVERSING_HACKERSPACE_2P: MessengerTraversingHackerspace2Piece = MessengerTraversingHackerspace2Piece;
pub static MESSENGER_TRAVERSING_HACKERSPACE_4P: MessengerTraversingHackerspace4Piece = MessengerTraversingHackerspace4Piece;

impl RelicSetKit for MessengerTraversingHackerspace2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.spd_pct += 0.06;
    }
}

impl RelicSetKit for MessengerTraversingHackerspace4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.messenger_traversing_hackerspace_4p {
            p.boosts.spd_pct += 0.12;
        }
    }
}
