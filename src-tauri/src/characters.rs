pub mod jingliu;

use crate::{data::{use_character_trace_node, CharacterDescriptor, EffectPropertyType, Element}, damage::{Boosts, EnemyConfig, CharacterStats}, promotions::CharacterState};

#[derive(Debug, Clone)]
pub struct CharacterTraceIds {
    pub ability_1: String,
    pub ability_2: String,
    pub ability_3: String,
    pub stat_1   : String,
    pub stat_2   : String,
    pub stat_3   : String,
    pub stat_4   : String,
    pub stat_5   : String,
    pub stat_6   : String,
    pub stat_7   : String,
    pub stat_8   : String,
    pub stat_9   : String,
    pub stat_10  : String,
}

impl CharacterTraceIds {
    pub fn from_character(character: &CharacterDescriptor) -> Self {
        let mut trace_ids: Vec<&str> = Vec::new();
        for trace_id in &character.skill_trees {
            let trace = use_character_trace_node(&trace_id);
            if trace.name != "" {
                trace_ids.push(trace_id);
            }
        }

        assert_eq!(trace_ids.len(), 13);

        return Self {
            ability_1: trace_ids[0].to_owned(),
            ability_2: trace_ids[1].to_owned(),
            ability_3: trace_ids[2].to_owned(),
            stat_1   : trace_ids[3].to_owned(),
            stat_2   : trace_ids[4].to_owned(),
            stat_3   : trace_ids[5].to_owned(),
            stat_4   : trace_ids[6].to_owned(),
            stat_5   : trace_ids[7].to_owned(),
            stat_6   : trace_ids[8].to_owned(),
            stat_7   : trace_ids[9].to_owned(),
            stat_8   : trace_ids[10].to_owned(),
            stat_9   : trace_ids[11].to_owned(),
            stat_10  : trace_ids[12].to_owned(),
        };
    }
}

fn apply_std_trace_effect(effective_element: Element, trace_id: &str, boosts: &mut Boosts) {
    let trace = use_character_trace_node(trace_id);
    for effect in &trace.levels[0].properties {
        apply_effect_boost(effective_element, effect.property_type, effect.value, boosts);
    }
}

pub fn apply_effect_boost(effective_element: Element, effect: EffectPropertyType, value: f64, boosts: &mut Boosts) {
    match effect {
        EffectPropertyType::HPDelta                   => boosts.hp_flat += value,
        EffectPropertyType::AttackDelta               => boosts.atk_flat += value,
        EffectPropertyType::DefenceDelta              => boosts.def_flat += value,
        EffectPropertyType::SpeedDelta                => boosts.spd += value,
        EffectPropertyType::HPAddedRatio              => boosts.hp_pct += value,
        EffectPropertyType::AttackAddedRatio          => boosts.atk_pct += value,
        EffectPropertyType::DefenceAddedRatio         => boosts.def_pct += value,
        EffectPropertyType::CriticalChanceBase        => boosts.crit_rate += value,
        EffectPropertyType::CriticalDamageBase        => boosts.crit_dmg += value,
        EffectPropertyType::HealRatioBase             => boosts.outgoing_healing_boost += value,
        EffectPropertyType::StatusProbabilityBase     => boosts.effect_hit_rate += value,
        EffectPropertyType::BreakDamageAddedRatioBase => boosts.break_effect += value,
        EffectPropertyType::SPRatioBase               => boosts.energy_recharge += value,
        EffectPropertyType::StatusResistanceBase      => boosts.effect_res += value,

        EffectPropertyType::AllDamageTypeAddedRatio   => boosts.all_type_dmg_boost += value,
        EffectPropertyType::PhysicalAddedRatio        => if effective_element == Element::Physical  { boosts.elemental_dmg_boost += value },
        EffectPropertyType::FireAddedRatio            => if effective_element == Element::Fire      { boosts.elemental_dmg_boost += value },
        EffectPropertyType::IceAddedRatio             => if effective_element == Element::Ice       { boosts.elemental_dmg_boost += value },
        EffectPropertyType::ThunderAddedRatio         => if effective_element == Element::Thunder   { boosts.elemental_dmg_boost += value },
        EffectPropertyType::WindAddedRatio            => if effective_element == Element::Wind      { boosts.elemental_dmg_boost += value },
        EffectPropertyType::QuantumAddedRatio         => if effective_element == Element::Quantum   { boosts.elemental_dmg_boost += value },
        EffectPropertyType::ImaginaryAddedRatio       => if effective_element == Element::Imaginary { boosts.elemental_dmg_boost += value },
    }
}

pub fn apply_minor_trace_effects(character: &CharacterDescriptor, character_state: &CharacterState, boosts: &mut Boosts) {
    let trace_ids = CharacterTraceIds::from_character(character);

    // Major traces are applied in the character kit, since they are character-specific
    // But minor traces can be done automatically

    let el = character.element;
    if character_state.traces.stat_1  { apply_std_trace_effect(el, &trace_ids.stat_1,  boosts); }
    if character_state.traces.stat_2  { apply_std_trace_effect(el, &trace_ids.stat_2,  boosts); }
    if character_state.traces.stat_3  { apply_std_trace_effect(el, &trace_ids.stat_3,  boosts); }
    if character_state.traces.stat_4  { apply_std_trace_effect(el, &trace_ids.stat_4,  boosts); }
    if character_state.traces.stat_5  { apply_std_trace_effect(el, &trace_ids.stat_5,  boosts); }
    if character_state.traces.stat_6  { apply_std_trace_effect(el, &trace_ids.stat_6,  boosts); }
    if character_state.traces.stat_7  { apply_std_trace_effect(el, &trace_ids.stat_7,  boosts); }
    if character_state.traces.stat_8  { apply_std_trace_effect(el, &trace_ids.stat_8,  boosts); }
    if character_state.traces.stat_9  { apply_std_trace_effect(el, &trace_ids.stat_9,  boosts); }
    if character_state.traces.stat_10 { apply_std_trace_effect(el, &trace_ids.stat_10, boosts); }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatColumnType {
    BasicDamage,
    SkillDamage,
    SkillHeal,
    SkillShield,
    UltimateDamage
}

impl StatColumnType {
    pub fn to_name(&self) -> &'static str {
        match self {
            StatColumnType::BasicDamage => "Basic DMG",
            StatColumnType::SkillDamage => "Skill DMG",
            StatColumnType::SkillHeal => "Skill Heal",
            StatColumnType::SkillShield => "Skill Shield",
            StatColumnType::UltimateDamage => "Ultimate DMG",
        }
    }
}

// pub struct StatColumn<Kit: ?Sized> {
//     pub stat_type: StatColumnType,
//     pub computer: fn(&Kit, &CharacterState, &CharacterStats, &Boosts, &EnemyConfig) -> f64,
// }

pub trait CharacterKit {
    fn apply_static_passives(&self, enemy_config: &EnemyConfig, character_state: &CharacterState, boosts: &mut Boosts);
    fn apply_conditional_passives(&self, enemy_config: &EnemyConfig, stat_type: StatColumnType, character_state: &CharacterState, boosts: &mut Boosts);

    fn get_stat_columns(&self) -> Vec<StatColumnType>;
    fn compute_stat_column(&self, column_type: StatColumnType, character_state: &CharacterState, character_stats: &CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64;
}

