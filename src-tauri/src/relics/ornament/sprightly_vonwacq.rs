use crate::{RelicSetKit, RelicSetKitParams};

pub struct SprightlyVonwacq;
pub static SPRIGHTLY_VONWACQ: SprightlyVonwacq = SprightlyVonwacq;

impl RelicSetKit for SprightlyVonwacq {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.energy_recharge += 0.05;
    }
}
