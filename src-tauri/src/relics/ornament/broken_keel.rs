use crate::{RelicSetKit, RelicSetKitParams};

pub struct BrokenKeel;
pub static BROKEN_KEEL: BrokenKeel = BrokenKeel;

impl RelicSetKit for BrokenKeel {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.effect_res += 0.10;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.boosts.effect_res >= 0.30 {
            p.boosts.crit_dmg += 0.10;
        }
    }
}
