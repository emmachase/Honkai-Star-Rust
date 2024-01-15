use crate::{data_mappings::Character, data::{use_character, use_character_trace_node, CharacterDescriptor, EffectPropertyType}, damage::{Boosts, EnemyConfig, CharacterStats}, promotions::CharacterState};

#[derive(Debug, Clone, Copy)]
pub struct CharacterTraceIds {
    pub ability_1: &'static str,
    pub ability_2: &'static str,
    pub ability_3: &'static str,
    pub stat_1   : &'static str,
    pub stat_2   : &'static str,
    pub stat_3   : &'static str,
    pub stat_4   : &'static str,
    pub stat_5   : &'static str,
    pub stat_6   : &'static str,
    pub stat_7   : &'static str,
    pub stat_8   : &'static str,
    pub stat_9   : &'static str,
    pub stat_10  : &'static str,
}

impl CharacterTraceIds {
    pub fn from_character(character: &'static CharacterDescriptor) -> Self {
        let mut trace_ids: Vec<&str> = Vec::new();
        for trace_id in &character.skill_trees {
            let trace = use_character_trace_node(&trace_id);
            if trace.name != "" {
                trace_ids.push(trace_id);
            }
        }

        assert_eq!(trace_ids.len(), 13);

        return Self {
            ability_1: trace_ids[0],
            ability_2: trace_ids[1],
            ability_3: trace_ids[2],
            stat_1   : trace_ids[3],
            stat_2   : trace_ids[4],
            stat_3   : trace_ids[5],
            stat_4   : trace_ids[6],
            stat_5   : trace_ids[7],
            stat_6   : trace_ids[8],
            stat_7   : trace_ids[9],
            stat_8   : trace_ids[10],
            stat_9   : trace_ids[11],
            stat_10  : trace_ids[12],
        };
    }
}

fn apply_std_trace_effect(trace_id: &str, boosts: &mut Boosts) {
    let trace = use_character_trace_node(trace_id);
    for effect in &trace.levels[0].properties {
        match effect.property_type {
            EffectPropertyType::HPDelta                   => boosts.hp_flat += effect.value,
            EffectPropertyType::AttackDelta               => boosts.atk_flat += effect.value,
            EffectPropertyType::DefenceDelta              => boosts.def_flat += effect.value,
            EffectPropertyType::SpeedDelta                => boosts.spd += effect.value,
            EffectPropertyType::HPAddedRatio              => boosts.hp_pct += effect.value,
            EffectPropertyType::AttackAddedRatio          => boosts.atk_pct += effect.value,
            EffectPropertyType::DefenceAddedRatio         => boosts.def_pct += effect.value,
            EffectPropertyType::CriticalChanceBase        => boosts.crit_rate += effect.value,
            EffectPropertyType::CriticalDamageBase        => boosts.crit_dmg += effect.value,
            EffectPropertyType::HealRatioBase             => boosts.outgoing_healing_boost += effect.value,
            EffectPropertyType::StatusProbabilityBase     => boosts.effect_hit_rate += effect.value,
            EffectPropertyType::PhysicalAddedRatio         |
            EffectPropertyType::FireAddedRatio             |
            EffectPropertyType::IceAddedRatio              |
            EffectPropertyType::ThunderAddedRatio          |
            EffectPropertyType::WindAddedRatio             |
            EffectPropertyType::QuantumAddedRatio          |
            EffectPropertyType::ImaginaryAddedRatio       => boosts.elemental_dmg_boost += effect.value,
            EffectPropertyType::AllDamageTypeAddedRatio   => boosts.all_type_dmg_boost += effect.value,
            EffectPropertyType::BreakDamageAddedRatioBase => boosts.break_effect += effect.value,
            EffectPropertyType::SPRatioBase               => boosts.energy_recharge += effect.value,
            EffectPropertyType::StatusResistanceBase      => boosts.effect_res += effect.value,
        }
    }
}

pub fn apply_minor_trace_effects(character: &'static CharacterDescriptor, character_state: &CharacterState, boosts: &mut Boosts) {
    let trace_ids = CharacterTraceIds::from_character(character);

    // Major traces are applied in the character kit, since they are character-specific
    // But minor traces can be done automatically

    if character_state.traces.stat_1  { apply_std_trace_effect(trace_ids.stat_1,  boosts); }
    if character_state.traces.stat_2  { apply_std_trace_effect(trace_ids.stat_2,  boosts); }
    if character_state.traces.stat_3  { apply_std_trace_effect(trace_ids.stat_3,  boosts); }
    if character_state.traces.stat_4  { apply_std_trace_effect(trace_ids.stat_4,  boosts); }
    if character_state.traces.stat_5  { apply_std_trace_effect(trace_ids.stat_5,  boosts); }
    if character_state.traces.stat_6  { apply_std_trace_effect(trace_ids.stat_6,  boosts); }
    if character_state.traces.stat_7  { apply_std_trace_effect(trace_ids.stat_7,  boosts); }
    if character_state.traces.stat_8  { apply_std_trace_effect(trace_ids.stat_8,  boosts); }
    if character_state.traces.stat_9  { apply_std_trace_effect(trace_ids.stat_9,  boosts); }
    if character_state.traces.stat_10 { apply_std_trace_effect(trace_ids.stat_10, boosts); }
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
