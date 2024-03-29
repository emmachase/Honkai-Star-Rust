         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async prankHimJohn(relics: Relic[], characterCfg: CharacterConfig, characterState: CharacterState, lightCone: [LightConeConfig, LightConeState] | null, enemyConfig: EnemyConfig, filters: StatFilter[]) : Promise<SortResultsSerde> {
return await TAURI_INVOKE("plugin:tauri-specta|prank_him_john", { relics, characterCfg, characterState, lightCone, enemyConfig, filters });
},
async stopPranking() : Promise<null> {
return await TAURI_INVOKE("plugin:tauri-specta|stop_pranking");
},
async parseKelz(scan: string) : Promise<__Result__<{ id: string; set: RelicSet; slot: RelicSlot; level: number; main_stat: [EffectPropertyType, number]; sub_stats: ([EffectPropertyType, number])[] }[], string>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:tauri-specta|parse_kelz", { scan }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getDescription(character: Character) : Promise<CharacterDescriptions> {
return await TAURI_INVOKE("plugin:tauri-specta|get_description", { character });
},
async getCharPreview(character: Character) : Promise<string> {
return await TAURI_INVOKE("plugin:tauri-specta|get_char_preview", { character });
},
async getLcIcon(lightCone: LightCone) : Promise<string> {
return await TAURI_INVOKE("plugin:tauri-specta|get_lc_icon", { lightCone });
},
async getLcPreview(lightCone: LightCone) : Promise<string> {
return await TAURI_INVOKE("plugin:tauri-specta|get_lc_preview", { lightCone });
},
async getEidolonUpgrades(character: Character) : Promise<EidolonUpgrade[]> {
return await TAURI_INVOKE("plugin:tauri-specta|get_eidolon_upgrades", { character });
},
async getCharacterActions(characterCfg: CharacterConfig) : Promise<([StatColumnType, string])[]> {
return await TAURI_INVOKE("plugin:tauri-specta|get_character_actions", { characterCfg });
}
}



/** user-defined types **/

export type Character = "March7th" | "DanHeng" | "Himeko" | "Welt" | "Kafka" | "SilverWolf" | "Arlan" | "Asta" | "Herta" | "Bronya" | "Seele" | "Serval" | "Gepard" | "Natasha" | "Pela" | "Clara" | "Sampo" | "Hook" | "Lynx" | "Luka" | "TopazAndNumby" | "Qingque" | "Tingyun" | "Luocha" | "JingYuan" | "Blade" | "Sushang" | "Yukong" | "FuXuan" | "Yanqing" | "Guinaifen" | "Bailu" | "Jingliu" | "DanHengImbibitorLunae" | "Xueyi" | "Hanya" | "Huohuo" | "Argenti" | "RuanMei" | "DrRatio" | "Sparkle" | "BlackSwan" | "Misha" | "PhysicalTrailblazerM" | "PhysicalTrailblazerF" | "FireTrailblazerM" | "FireTrailblazerF"
export type CharacterConfig = { Jingliu: JingliuConfig } | { Sparkle: SparkleConfig }
export type CharacterDescriptions = { Jingliu: JingliuDescriptions } | { Sparkle: SparkleDescriptions }
export type CharacterSkillState = { basic: number; skill: number; ult: number; talent: number }
export type CharacterState = { level: number; ascension: number; eidolon: number; skills: CharacterSkillState; traces: CharacterTraceState }
export type CharacterStats = { level: number; ascension: number; element: Element; hp: number; atk: number; def: number; spd: number; effect_res: number; crit_rate: number; crit_dmg: number; break_effect: number; energy_recharge: number; outgoing_healing_boost: number; elemental_dmg_boost: ElementalDmgBoost; effect_hit_rate: number }
export type CharacterTraceState = { ability_1: boolean; ability_2: boolean; ability_3: boolean; stat_1: boolean; stat_2: boolean; stat_3: boolean; stat_4: boolean; stat_5: boolean; stat_6: boolean; stat_7: boolean; stat_8: boolean; stat_9: boolean; stat_10: boolean }
export type EarthlyEscapadeConfig = { has_mask: boolean }
export type EffectPropertyType = "HPDelta" | "AttackDelta" | "DefenceDelta" | "SpeedDelta" | "HPAddedRatio" | "AttackAddedRatio" | "DefenceAddedRatio" | "CriticalChanceBase" | "CriticalDamageBase" | "HealRatioBase" | "StatusProbabilityBase" | "PhysicalAddedRatio" | "FireAddedRatio" | "IceAddedRatio" | "ThunderAddedRatio" | "WindAddedRatio" | "QuantumAddedRatio" | "ImaginaryAddedRatio" | "AllDamageTypeAddedRatio" | "BreakDamageAddedRatioBase" | "SPRatioBase" | "StatusResistanceBase"
export type EidolonUpgrade = { basic: number; skill: number; ult: number; talent: number }
export type Element = "Physical" | "Fire" | "Ice" | "Thunder" | "Wind" | "Quantum" | "Imaginary"
export type ElementalDmgBoost = [number, number, number, number, number, number, number]
export type EnemyConfig = { count: number; level: number; resistance: number; elemental_weakness: boolean; weakness_broken: boolean; debuff_count: number }
export type IShallBeMyOwnSwordConfig = { eclipse_stacks: number; max_stack_def_pen: boolean }
/**
 * 
 * * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy.
 * 
 */
export type JingliuBasicDesc = { atk_pct: number }
export type JingliuConfig = { enhanced_state: boolean; hp_drain_pct: number; e1_crit_dmg: boolean; e2_skill_buff: boolean }
export type JingliuDescriptions = { basic: JingliuBasicDesc[]; normal_skill: JingliuNormalSkillDesc[]; ultimate: JingliuUltimateDesc[]; talent: JingliuTalentDesc[]; enhanced_skill: JingliuEnhancedSkillDesc[] }
/**
 * 
 * * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal
 * * to #3[i]% of Jingliu's ATK to adjacent enemies. Consumes #2[i] stack(s) of Syzygy. Using
 * * this ability does not consume Skill Points.
 * 
 */
export type JingliuEnhancedSkillDesc = { atk_pct_main: number; _syzygy_stacks: number; _atk_pct_adj: number }
/**
 * 
 * * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy and obtains #2[i] stack(s) of Syzygy.
 * 
 */
export type JingliuNormalSkillDesc = { atk_pct: number; _syzygy_stacks: number }
/**
 * 
 * * When Jingliu has #5[i] stack(s) of Syzygy, she enters the Spectral Transmigration state with her
 * * Action Advanced by #6[i]% and her CRIT Rate increases by #7[i]%. Then, Jingliu's Skill
 * * \"Transcendent Flash\" is enhanced to \"Moon On Glacial River,\" and only this enhanced Skill is
 * * available for use in battle. When Jingliu uses an attack in the Spectral Transmigration state,
 * * she consumes HP from all other allies equal to #2[i]% of their respective Max HP (this cannot
 * * reduce allies' HP to lower than 1). Jingliu's ATK increases by #3[i]% of the total HP consumed
 * * from all allies in this attack, capped at #4[i]% of her base ATK, lasting until the current attack
 * * ends. Jingliu cannot enter the Spectral Transmigration state again until the current
 * * Spectral Transmigration state ends. Syzygy can stack up to 3 times. When Syzygy stacks become 0,
 * * Jingliu will exit the Spectral Transmigration state.
 * 
 */
export type JingliuTalentDesc = { _unknown: number; _consume_hp_pct: number; _atk_pct_from_hp: number; atk_pct_cap: number; _required_stacks: number; _action_advance_pct: number; crit_rate_pct: number }
/**
 * 
 * * Deals Ice DMG equal to #1[i]% of Jingliu's ATK to a single enemy, and deals Ice DMG equal to #3[i]%
 * * of Jingliu's ATK to any adjacent enemies. Gains #2[i] stack(s) of Syzygy after attack ends.
 * 
 */
export type JingliuUltimateDesc = { atk_pct_main: number; _syzygy_stacks: number; _atk_pct_adj: number; _unknown: number }
export type LightCone = "Arrows" | "Cornucopia" | "CollapsingSky" | "Amber" | "Void" | "Chorus" | "DataBank" | "DartingArrow" | "FineFruit" | "ShatteredHome" | "Defense" | "Loop" | "MeshingCogs" | "Passkey" | "Adversarial" | "Multiplication" | "MutualDemise" | "Pioneering" | "HiddenShadow" | "Mediation" | "Sagacity" | "PostOpConversation" | "GoodNightAndSleepWell" | "DayOneOfMyNewLife" | "OnlySilenceRemains" | "MemoriesOfThePast" | "TheMolesWelcomeYou" | "TheBirthOfTheSelf" | "SharedFeeling" | "EyesOfThePrey" | "LandauSChoice" | "Swordplay" | "PlanetaryRendezvous" | "ASecretVow" | "MakeTheWorldClamor" | "PerfectTiming" | "ResolutionShinesAsPearlsOfSweat" | "TrendOfTheUniversalMarket" | "SubscribeForMore" | "DanceDanceDance" | "UnderTheBlueSky" | "GeniusesRepose" | "QuidProQuo" | "Fermata" | "WeAreWildfire" | "RiverFlowsInSpring" | "PastAndFuture" | "WoofWalkTime" | "TheSeriousnessOfBreakfast" | "WarmthShortensColdNights" | "WeWillMeetAgain" | "ThisIsMe" | "ReturnToDarkness" | "CarveTheMoonWeaveTheClouds" | "NowhereToRun" | "TodayIsAnotherPeacefulDay" | "WhatIsReal" | "DreamvilleAdventure" | "FinalVictor" | "FlamesAfar" | "DestinySThreadsForewoven" | "TheDayTheCosmosFell" | "ItSShowtime" | "IndeliblePromise" | "BeforeTheTutorialMissionStarts" | "HeyOverHere" | "NightOnTheMilkyWay" | "InTheNight" | "SomethingIrreplaceable" | "ButTheBattleIsnTOver" | "InTheNameOfTheWorld" | "MomentOfVictory" | "PatienceIsAllYouNeed" | "IncessantRain" | "EchoesOfTheCoffin" | "TheUnreachableSide" | "BeforeDawn" | "SheAlreadyShutHerEyes" | "SleepLikeTheDead" | "TimeWaitsForNoOne" | "IShallBeMyOwnSword" | "BrighterThanTheSun" | "WorrisomeBlissful" | "NightOfFright" | "AnInstantBeforeAGaze" | "PastSelfInMirror" | "BaptismOfPureThought" | "EarthlyEscapade" | "ReforgedRemembrance" | "OnTheFallOfAnAeon" | "CruisingInTheStellarSea" | "TextureOfMemories" | "SolitaryHealing"
export type LightConeConfig = { IShallBeMyOwnSword: IShallBeMyOwnSwordConfig } | { EarthlyEscapade: EarthlyEscapadeConfig }
export type LightConeState = { level: number; ascension: number; superimposition: number }
export type Relic = { id: string; set: RelicSet; slot: RelicSlot; level: number; main_stat: [EffectPropertyType, number]; sub_stats: ([EffectPropertyType, number])[] }
export type RelicSet = "PasserbyOfWanderingCloud" | "MusketeerOfWildWheat" | "KnightOfPurityPalace" | "HunterOfGlacialForest" | "ChampionOfStreetwiseBoxing" | "GuardOfWutheringSnow" | "FiresmithOfLavaForging" | "GeniusOfBrilliantStars" | "BandOfSizzlingThunder" | "EagleOfTwilightLine" | "ThiefOfShootingMeteor" | "WastelanderOfBanditryDesert" | "LongevousDisciple" | "MessengerTraversingHackerspace" | "TheAshblazingGrandDuke" | "PrisonerInDeepConfinement" | "PioneerDiverOfDeadWaters" | "WatchmakerMasterOfDreamMachinations" | "SpaceSealingStation" | "FleetOfTheAgeless" | "PanCosmicCommercialEnterprise" | "BelobogOfTheArchitects" | "CelestialDifferentiator" | "InertSalsotto" | "TaliaKingdomOfBanditry" | "SprightlyVonwacq" | "RutilantArena" | "BrokenKeel" | "FirmamentFrontlineGlamoth" | "PenaconyLandOfTheDreams"
export type RelicSlot = "Head" | "Hands" | "Chest" | "Feet" | "PlanarSphere" | "LinkRope"
export type ResolvedCalculatorResult = { relic_perm: string[]; cols: ([string, number])[]; calculated_stats: [CharacterStats, CharacterStats] }
export type SortResultsSerde = { effective_element: Element; base: SortResultsSerdeBase; combat: SortResultsSerdeBase; cols: ([string, ResolvedCalculatorResult[]])[] }
export type SortResultsSerdeBase = { hp: ResolvedCalculatorResult[]; atk: ResolvedCalculatorResult[]; def: ResolvedCalculatorResult[]; spd: ResolvedCalculatorResult[]; effect_res: ResolvedCalculatorResult[]; crit_rate: ResolvedCalculatorResult[]; crit_dmg: ResolvedCalculatorResult[]; break_effect: ResolvedCalculatorResult[]; energy_recharge: ResolvedCalculatorResult[]; outgoing_healing_boost: ResolvedCalculatorResult[]; elemental_dmg_boost: ResolvedCalculatorResult[]; effect_hit_rate: ResolvedCalculatorResult[] }
export type SparkleBaseConfig = { skill_cd_buff: boolean; cipher_buff: boolean; talent_dmg_stacks: number; quantum_allies: number }
/**
 * 
 * * Deals Quantum DMG equal to #1[i]% of Sparkle's ATK to a single enemy.
 * 
 */
export type SparkleBasicDesc = { atk_pct: number }
export type SparkleConfig = { Own: SparkleBaseConfig } | { Teammate: SparkleTeammateConfig }
export type SparkleDescriptions = { basic: SparkleBasicDesc[]; skill: SparkleSkillDesc[]; ultimate: SparkleUltimateDesc[]; talent: SparkleTalentDesc[] }
/**
 * 
 * * Increases the CRIT DMG of a single target ally by #1[f1]% of Sparkle's
 * * CRIT DMG plus #2[f1]%, lasting for #3[i] turn(s). And at the same time,
 * * Advances Forward this ally's action by #4[i]%.\nWhen Sparkle uses this
 * * ability on herself, the Action Advance effect will not trigger.
 * 
 */
export type SparkleSkillDesc = { crit_dmg_pct: number; crit_dmg_flat: number; duration: number; action_advance: number }
/**
 * 
 * * While Sparkle is on the battlefield, additionally increases the max number
 * * of Skill Points by #3[i]. Whenever an ally consumes 1 Skill Point, all
 * * allies' DMG dealt increases by #2[f1]%. This effect lasts for #1[i] turn(s)
 * * and can stack up to #4[i] time(s).
 * 
 */
export type SparkleTalentDesc = { duration: number; dmg_boost_pct: number; skill_points: number; stacks: number }
export type SparkleTeammateConfig = { cd_stat: number; skill_cd_buff: boolean; cipher_buff: boolean; talent_dmg_stacks: number; quantum_allies: number }
/**
 * 
 * * Recovers #2[i] Skill Points for the team and grants all allies Cipher.
 * * For allies with Cipher, each stack of the DMG Boost effect provided by
 * * Sparkle's Talent additionally increases by #3[f1]%, lasting for #4[i]
 * * turns.
 * 
 */
export type SparkleUltimateDesc = { _unknown: number; skill_points: number; dmg_boost_pct: number; duration: number }
export type StatColumnType = "BasicDamage" | "SkillDamage" | "SkillHeal" | "SkillShield" | "UltimateDamage" | "UltimateHeal" | "UltimateShield" | "FollowUpDamage"
export type StatFilter = { HP: [StatFilterType, number | null, number | null] } | { ATK: [StatFilterType, number | null, number | null] } | { DEF: [StatFilterType, number | null, number | null] } | { SPD: [StatFilterType, number | null, number | null] } | { EffectRes: [StatFilterType, number | null, number | null] } | { CritRate: [StatFilterType, number | null, number | null] } | { CritDmg: [StatFilterType, number | null, number | null] } | { BreakEffect: [StatFilterType, number | null, number | null] } | { EnergyRecharge: [StatFilterType, number | null, number | null] } | { OutgoingHealingBoost: [StatFilterType, number | null, number | null] } | { ElementalDmgBoost: [StatFilterType, number | null, number | null] } | { EffectHitRate: [StatFilterType, number | null, number | null] } | { CritValue: [StatFilterType, number | null, number | null] } | { EffectiveHP: [StatFilterType, number | null, number | null] } | { Weight: [StatFilterType, number | null, number | null] } | { Action: [StatColumnType, number | null, number | null] }
export type StatFilterType = "Base" | "Combat"

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindowHandle as __WebviewWindowHandle__ } from "@tauri-apps/api/window";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

type __Result__<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindowHandle__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     