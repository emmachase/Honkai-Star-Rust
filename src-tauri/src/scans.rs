use serde::{de, Deserialize};
use lazy_static::lazy_static;
use specta::Type;
use uuid::Uuid;

use crate::{data_mappings::{RelicSet, Character}, data::{RelicSlot, EffectPropertyType, use_relic_main_affixes}, relics::{Relic, RelicStat}};

const TEST_SCAN_JSON: &str = include_str!("./data/scans/test.json");

fn deserialize_kelz_set<'de, D>(deserializer: D) -> Result<RelicSet, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    RelicSet::from_name(&s).ok_or(de::Error::custom("invalid relic set name"))
}

fn deserialize_kelz_relic_slot<'de, D>(deserializer: D) -> Result<RelicSlot, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "Head"          => Ok(RelicSlot::Head),
        "Hands"         => Ok(RelicSlot::Hands),
        "Body"          => Ok(RelicSlot::Chest),
        "Feet"          => Ok(RelicSlot::Feet),
        "Planar Sphere" => Ok(RelicSlot::PlanarSphere),
        "Link Rope"     => Ok(RelicSlot::LinkRope),
        _ => Err(de::Error::custom("invalid relic slot name")),
    }
}

fn deserialize_kelz_character<'de, D>(deserializer: D) -> Result<Option<Character>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    if s == "" {
        return Ok(None);
    }

    Character::from_name(&s).ok_or(de::Error::custom(format!("invalid character name {}", s))).map(Some)
}

fn from_stat_name(s: &str, slot: RelicSlot, is_main: bool) -> Option<EffectPropertyType> {
    let is_pct_main = match slot {
        RelicSlot::Head | RelicSlot::Hands => false,
        _ => is_main,
    };

    match s {
        "HP"  if is_pct_main => Some(EffectPropertyType::HPAddedRatio),
        "ATK" if is_pct_main => Some(EffectPropertyType::AttackAddedRatio),
        "DEF" if is_pct_main => Some(EffectPropertyType::DefenceAddedRatio),

        "HP"  => Some(EffectPropertyType::HPDelta),
        "ATK" => Some(EffectPropertyType::AttackDelta),
        "DEF" => Some(EffectPropertyType::DefenceDelta),
        "SPD" => Some(EffectPropertyType::SpeedDelta),

        "HP_"  => Some(EffectPropertyType::HPAddedRatio),
        "ATK_" => Some(EffectPropertyType::AttackAddedRatio),
        "DEF_" => Some(EffectPropertyType::DefenceAddedRatio),

        "CRIT Rate_" | "CRIT Rate" => Some(EffectPropertyType::CriticalChanceBase),
        "CRIT DMG_"  | "CRIT DMG"  => Some(EffectPropertyType::CriticalDamageBase),
        "Outgoing Healing Boost"   => Some(EffectPropertyType::HealRatioBase),
        "Effect Hit Rate_" |
        "Effect Hit Rate"          => Some(EffectPropertyType::StatusProbabilityBase),

        "Physical DMG Boost"  => Some(EffectPropertyType::PhysicalAddedRatio),
        "Fire DMG Boost"      => Some(EffectPropertyType::FireAddedRatio),
        "Ice DMG Boost"       => Some(EffectPropertyType::IceAddedRatio),
        "Lightning DMG Boost" => Some(EffectPropertyType::ThunderAddedRatio),
        "Wind DMG Boost"      => Some(EffectPropertyType::WindAddedRatio),
        "Quantum DMG Boost"   => Some(EffectPropertyType::QuantumAddedRatio),
        "Imaginary DMG Boost" => Some(EffectPropertyType::ImaginaryAddedRatio),
        // "" => Some(EffectPropertyType::AllDamageTypeAddedRatio),

        "Break Effect" |
        "Break Effect_"            => Some(EffectPropertyType::BreakDamageAddedRatioBase),
        "Energy Regeneration Rate" => Some(EffectPropertyType::SPRatioBase),
        "Effect RES_"              => Some(EffectPropertyType::StatusResistanceBase),
        _ => None,
    }
}

#[derive(Debug, Deserialize, Type)]
pub struct KelZSubstat {
    pub key: String,
    pub value: f64,
}

#[derive(Debug, Deserialize, Type)]
pub struct KelZRelic {
    #[serde(deserialize_with = "deserialize_kelz_set")]
    pub set: RelicSet,
    #[serde(deserialize_with = "deserialize_kelz_relic_slot")]
    pub slot: RelicSlot,
    pub rarity: u8,
    pub level: u8,
    #[serde(rename = "mainstat")]
    pub main_stat: String,
    #[serde(rename = "substats")]
    pub sub_stats: Vec<KelZSubstat>,
    #[serde(deserialize_with = "deserialize_kelz_character")]
    pub location: Option<Character>,
    pub lock: bool,
    pub _id: String,
}

impl KelZRelic {
    pub fn to_relic(&self) -> Option<Relic> {
        let main_stat = self.calculate_main_affix()?;
        let mut sub_stats: Vec<RelicStat> = vec![];
        for sub_stat in &self.sub_stats {
            let stat_type = from_stat_name(&sub_stat.key, self.slot, false)?;
            sub_stats.push((stat_type, if stat_type.is_pct() { sub_stat.value / 100.0 } else { sub_stat.value }));
        }

        return Some(Relic {
            id: Uuid::new_v4().to_string(),

            set: self.set,
            slot: self.slot,
            level: self.level,
            main_stat,
            sub_stats,
        });
    }

    fn calculate_main_affix(&self) -> Option<RelicStat> {
        let main_stat = from_stat_name(&self.main_stat, self.slot, true)?;

        let affixes = &use_relic_main_affixes(self.rarity, self.slot).affixes;
        let affix = affixes.iter().find(|(_, desc)| desc.property == main_stat)?.1;

        let affix_value = affix.base + affix.step * self.level as f64;
        return Some((main_stat, affix_value));
    }
}

#[derive(Debug, Deserialize)]
pub struct KelZScan {
    pub source: String,
    pub version: u8,
    // pub light_cones: Vec<>,
    pub relics: Vec<KelZRelic>,
    // pub characters: Vec<>,
}

lazy_static! {
    pub static ref TEST_SCAN: KelZScan = serde_json::from_str(TEST_SCAN_JSON).unwrap();
}
