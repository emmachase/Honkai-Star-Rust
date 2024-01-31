use crate::{RelicSetKit, RelicSetKitParams};

pub struct BelobogOfTheArchitects;
pub static BELOBOG_OF_THE_ARCHITECTS: BelobogOfTheArchitects = BelobogOfTheArchitects;

impl RelicSetKit for BelobogOfTheArchitects {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.def_pct += 0.15;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.boosts.effect_hit_rate >= 0.50 {
            p.boosts.def_pct += 0.15;
        }
    }
}
