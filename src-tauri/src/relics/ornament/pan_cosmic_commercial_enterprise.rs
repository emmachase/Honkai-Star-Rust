use crate::{RelicSetKit, RelicSetKitParams};

pub struct PanCosmicCommercialEnterprise;
pub static PAN_COSMIC_COMMERCIAL_ENTERPRISE: PanCosmicCommercialEnterprise = PanCosmicCommercialEnterprise;

impl RelicSetKit for PanCosmicCommercialEnterprise {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.effect_hit_rate += 0.10;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += (0.25*p.boosts.effect_hit_rate).min(0.25);
    }
}
