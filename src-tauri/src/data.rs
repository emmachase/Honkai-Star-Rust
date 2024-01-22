use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::IntoDeserializer};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use lazy_static::lazy_static;
use specta::Type;

use crate::data_mappings::{Character, LightCone, RelicSet};

const CHARACTER_PROMOTIONS_JSON : &str = include_str!("./data/character_promotions.json");
const CHARACTER_SKILL_TREES_JSON: &str = include_str!("./data/character_skill_trees.json");
const CHARACTER_SKILLS_JSON     : &str = include_str!("./data/character_skills.json");
const CHARACTERS_JSON           : &str = include_str!("./data/characters.json");
const LIGHT_CONE_PROMOTIONS_JSON: &str = include_str!("./data/light_cone_promotions.json");
const LIGHT_CONE_RANKS_JSON     : &str = include_str!("./data/light_cone_ranks.json");
const LIGHT_CONES_JSON          : &str = include_str!("./data/light_cones.json");
const RELIC_SETS_JSON           : &str = include_str!("./data/relic_sets.json");
const RELIC_MAIN_AFFIXES_JSON   : &str = include_str!("./data/relic_main_affixes.json");
const RELIC_SUB_AFFIXES_JSON    : &str = include_str!("./data/relic_sub_affixes.json");

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PromotionStepSpec {
    pub base: f64,
    pub step: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterPromotionSpec {
    pub hp: PromotionStepSpec,
    pub atk: PromotionStepSpec,
    pub def: PromotionStepSpec,
    pub spd: PromotionStepSpec,
    pub taunt: PromotionStepSpec,
    pub crit_rate: PromotionStepSpec,
    pub crit_dmg: PromotionStepSpec,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightConePromotionSpec {
    pub hp: PromotionStepSpec,
    pub atk: PromotionStepSpec,
    pub def: PromotionStepSpec,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Promotions<Spec> {
    pub id: String, // Character ID
    pub values: Vec<Spec>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Thunder, // Lightning
    Wind,
    Quantum,
    Imaginary,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum CharacterInternalPath {
    Knight,  // Preservation
    Rogue,   // Hunt
    Mage,    // Erudition
    Warlock, // Nihility
    Warrior, // Destruction
    Shaman,  // Harmony
    Priest,  // Abundance
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CharacterDescriptor {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub rarity: u8,
    pub path: CharacterInternalPath,
    pub element: Element,
    pub max_sp: u8,               // Max energy for ultimate
    pub ranks: Vec<String>,       // Eidolon IDs
    pub skills: Vec<String>,      // Main Skill (Basic, Skill, Ult, etc) IDs
    pub skill_trees: Vec<String>, // Bonus Trace IDs
    pub icon: String,     // Cropped Icon path
    pub preview: String,  // Cropped Portrait path
    pub portrait: String, // Full Splash path
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightConeDescriptor {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub path: CharacterInternalPath,
    pub desc: String,
    pub icon: String,     // Inventory Icon path
    pub preview: String,  // Inventory Preview path
    pub portrait: String, // Full Card Art path
}

#[derive(Debug, Deserialize_tuple, Serialize_tuple)]
pub struct RelicSetEffects {
    pub two_piece: String,
    pub four_piece: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Type)]
pub enum EffectPropertyType {
    HPDelta,
    AttackDelta,
    DefenceDelta,
    SpeedDelta,

    HPAddedRatio,
    AttackAddedRatio,
    DefenceAddedRatio,

    CriticalChanceBase,
    CriticalDamageBase,
    HealRatioBase,
    StatusProbabilityBase,
    
    PhysicalAddedRatio,
    FireAddedRatio,
    IceAddedRatio,
    ThunderAddedRatio,
    WindAddedRatio,
    QuantumAddedRatio,
    ImaginaryAddedRatio,
    AllDamageTypeAddedRatio,
    
    BreakDamageAddedRatioBase,
    SPRatioBase,
    StatusResistanceBase,
}

impl EffectPropertyType {
    pub fn is_pct(&self) -> bool {
        match self {
            EffectPropertyType::HPDelta      |
            EffectPropertyType::AttackDelta  |
            EffectPropertyType::DefenceDelta |
            EffectPropertyType::SpeedDelta => false,
            _ => true
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EffectProperty {
    #[serde(rename = "type")]
    pub property_type: EffectPropertyType,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelicSetDescriptor {
    pub id: String,
    pub name: String,
    pub desc: RelicSetEffects,
    pub properties: Vec<Vec<EffectProperty>>,
    pub icon: String,
    pub guide_overview: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelicMainAffixDescriptor {
    pub affix_id: String,
    pub property: EffectPropertyType,
    pub base: f64,
    pub step: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelicSubAffixDescriptor {
    pub affix_id: String,
    pub property: EffectPropertyType,
    pub base: f64,
    pub step: f64,
    pub step_num: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelicAffixList<Descriptor> {
    pub id: String,
    pub affixes: HashMap<String, Descriptor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterTraceNodeLevel {
    pub promotion: u8,
    pub level: u8,
    pub properties: Vec<EffectProperty>,
    // pub materials: Vec<IdNameDescriptor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterTraceNode {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    pub desc: String,
    pub params: Vec<Vec<f64>>, // interpolation params for desc
    pub anchor: String,
    pub pre_points: Vec<String>,
    // pub level_up_skills: Vec<String>, // Default leveling?
    pub levels: Vec<CharacterTraceNodeLevel>,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SkillType {
    Normal,     // Basic Attack
    BPSkill,    // Skill
    Ultra,      // Ultimate
    Talent,     // Talent
    MazeNormal, // Attack and enter battle immediately
    Maze,       // Special effect
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SkillEffect {
    SingleAttack,
    Defence,
    AoEAttack,
    MazeAttack,
    Enhance,
    Blast,
    Impair,
    Bounce,
    Support,
    Restore,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterSkillDescriptor {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub element: Option<Element>,
    #[serde(rename = "type")]
    pub skill_type: SkillType,
    #[serde(rename = "type_text")]
    pub skill_type_text: String,
    pub effect: SkillEffect,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>, // interpolation params for desc
    pub icon: String,
}

impl CharacterSkillDescriptor {
    pub fn levels<T: for<'a> Deserialize<'a>>(&self) -> Vec<T> {
        return self.params.iter().map(|p| serde_json::from_value(serde_json::to_value(p).unwrap()).unwrap()).collect();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightConeEffectsDescriptor {
    pub id: String,
    pub skill: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>, // interpolation params for desc
    pub properties: Vec<Vec<EffectProperty>>,
}

impl LightConeEffectsDescriptor {
    pub fn superimpositions<T: for<'a> Deserialize<'a>>(&self) -> Vec<T> {
        return self.params.iter().map(|p| serde_json::from_value(serde_json::to_value(p).unwrap()).unwrap()).collect();
    }
}

lazy_static! {
    pub static ref CHARACTER_PROMOTIONS: HashMap<String, Promotions<CharacterPromotionSpec>> = serde_json::from_str(&CHARACTER_PROMOTIONS_JSON).unwrap();
    pub static ref CHARACTER_SKILL_TREES: HashMap<String, CharacterTraceNode> = serde_json::from_str(&CHARACTER_SKILL_TREES_JSON).unwrap();
    pub static ref CHARACTER_SKILLS: HashMap<String, CharacterSkillDescriptor> = serde_json::from_str(&CHARACTER_SKILLS_JSON).unwrap();
    pub static ref CHARACTERS: HashMap<String, CharacterDescriptor> = serde_json::from_str(&CHARACTERS_JSON).unwrap();
    pub static ref LIGHT_CONE_PROMOTIONS: HashMap<String, Promotions<LightConePromotionSpec>> = serde_json::from_str(&LIGHT_CONE_PROMOTIONS_JSON).unwrap();
    pub static ref LIGHT_CONE_RANKS: HashMap<String, LightConeEffectsDescriptor> = serde_json::from_str(&LIGHT_CONE_RANKS_JSON).unwrap();
    pub static ref LIGHT_CONES: HashMap<String, LightConeDescriptor> = serde_json::from_str(&LIGHT_CONES_JSON).unwrap();
    pub static ref RELIC_SETS: HashMap<String, RelicSetDescriptor> = serde_json::from_str(&RELIC_SETS_JSON).unwrap();
    pub static ref RELIC_MAIN_AFFIXES: HashMap<String, RelicAffixList<RelicMainAffixDescriptor>> = serde_json::from_str(&RELIC_MAIN_AFFIXES_JSON).unwrap();
    pub static ref RELIC_SUB_AFFIXES: HashMap<String, RelicAffixList<RelicSubAffixDescriptor>> = serde_json::from_str(&RELIC_SUB_AFFIXES_JSON).unwrap();
}

pub fn use_character_promotions(character: Character) -> &'static Promotions<CharacterPromotionSpec> {
    return &CHARACTER_PROMOTIONS[character.to_id()];
}

pub fn use_character_trace_node(trace_id: &str) -> &'static CharacterTraceNode {
    return &CHARACTER_SKILL_TREES[trace_id];
}

pub fn use_character_skill(skill_id: &str) -> &'static CharacterSkillDescriptor {
    return &CHARACTER_SKILLS[skill_id];
}

pub fn use_character(character: Character) -> &'static CharacterDescriptor {
    return &CHARACTERS[character.to_id()];
}

pub fn use_light_cone_promotions(light_cone: LightCone) -> &'static Promotions<LightConePromotionSpec> {
    return &LIGHT_CONE_PROMOTIONS[light_cone.to_id()];
}

pub fn use_light_cone_effects(light_cone: LightCone) -> &'static LightConeEffectsDescriptor {
    return &LIGHT_CONE_RANKS[light_cone.to_id()];
}

pub fn use_light_cone(light_cone: LightCone) -> &'static LightConeDescriptor {
    return &LIGHT_CONES[light_cone.to_id()];
}

pub fn use_relic_set(relic_set: RelicSet) -> &'static RelicSetDescriptor {
    return &RELIC_SETS[relic_set.to_id()];
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum RelicSlot {
    Head         = 1,
    Hands        = 2,
    Chest        = 3, 
    Feet         = 4,
    PlanarSphere = 5,
    LinkRope     = 6,
}

pub fn use_relic_main_affixes(rarity: u8, slot: RelicSlot) -> &'static RelicAffixList<RelicMainAffixDescriptor> {
    let id = format!("{}{}", rarity, slot as u8);
    return &RELIC_MAIN_AFFIXES[&id];
}

pub fn use_relic_sub_affixes(rarity: u8) -> &'static RelicAffixList<RelicSubAffixDescriptor> {
    return &RELIC_SUB_AFFIXES[&rarity.to_string()];
}
