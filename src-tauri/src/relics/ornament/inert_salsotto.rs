use crate::{characters::StatColumnType, RelicSetKit, RelicSetKitParams};

pub struct InertSalsotto;
pub static INERT_SALSOTTO: InertSalsotto = InertSalsotto;

impl RelicSetKit for InertSalsotto {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.crit_rate += 0.08;
    }

    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        match stat_type {
            StatColumnType::UltimateDamage |
            StatColumnType::FollowUpDamage => {
                if p.character_stats.crit_rate(&p.boosts) >= 0.50 {
                    p.boosts.all_type_dmg_boost += 0.15;
                }
            }

            _ => {}
        }
    }
}
