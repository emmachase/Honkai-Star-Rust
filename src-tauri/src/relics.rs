use crate::{data_mappings::RelicSet, data::{EffectPropertyType, RelicSlot, Element}, damage::{Level, Boosts}, characters::apply_effect_boost};

pub type RelicStat = (EffectPropertyType, f64);

pub struct Relic {
    pub set: RelicSet,
    pub slot: RelicSlot,
    pub level: Level,
    pub main_stat: RelicStat,
    pub sub_stats: Vec<RelicStat>,
}

pub struct RelicIterator<'a> {
    relic: &'a Relic,
    index: usize,
}

impl<'a> Iterator for RelicIterator<'a> {
    type Item = RelicStat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.index += 1;
            return Some(self.relic.main_stat);
        } else if self.index <= self.relic.sub_stats.len() {
            let index = self.index - 1;
            self.index += 1;
            return Some(self.relic.sub_stats[index]);
        } else {
            return None;
        }
    }
}

impl Relic {
    pub fn iter(&self) -> RelicIterator {
        return RelicIterator {
            relic: self,
            index: 0,
        };
    }

    pub fn apply(&self, effective_element: Element, boosts: &mut Boosts) {
        for (effect, value) in self.iter() {
            apply_effect_boost(effective_element, effect, value, boosts);
        }
    }
}
