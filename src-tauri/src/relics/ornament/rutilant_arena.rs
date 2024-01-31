use crate::{characters::StatColumnType, RelicSetKit, RelicSetKitParams};

pub struct RutilantArena;
pub static RUTILANT_ARENA: RutilantArena = RutilantArena;

impl RelicSetKit for RutilantArena {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.crit_rate += 0.08;
    }

    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        match stat_type {
            StatColumnType::BasicDamage |
            StatColumnType::SkillDamage => {
                if p.character_stats.crit_rate(&p.boosts) >= 0.70 {
                    p.boosts.all_type_dmg_boost += 0.20;
                }
            }

            _ => {}
        }
    }
}
