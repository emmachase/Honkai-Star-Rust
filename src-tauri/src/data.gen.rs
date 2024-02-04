// GENERATED CODE - DO NOT EDIT MANUALLY
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub enum RelicSet {
    PasserbyOfWanderingCloud,
    MusketeerOfWildWheat,
    KnightOfPurityPalace,
    HunterOfGlacialForest,
    ChampionOfStreetwiseBoxing,
    GuardOfWutheringSnow,
    FiresmithOfLavaForging,
    GeniusOfBrilliantStars,
    BandOfSizzlingThunder,
    EagleOfTwilightLine,
    ThiefOfShootingMeteor,
    WastelanderOfBanditryDesert,
    LongevousDisciple,
    MessengerTraversingHackerspace,
    TheAshblazingGrandDuke,
    PrisonerInDeepConfinement,
    SpaceSealingStation,
    FleetOfTheAgeless,
    PanCosmicCommercialEnterprise,
    BelobogOfTheArchitects,
    CelestialDifferentiator,
    InertSalsotto,
    TaliaKingdomOfBanditry,
    SprightlyVonwacq,
    RutilantArena,
    BrokenKeel,
    FirmamentFrontlineGlamoth,
    PenaconyLandOfTheDreams,
}

impl RelicSet {
    pub const COUNT: usize = 28;
    pub const fn to_id(self) -> &'static str {
        match self {
            RelicSet::PasserbyOfWanderingCloud => "101",
            RelicSet::MusketeerOfWildWheat => "102",
            RelicSet::KnightOfPurityPalace => "103",
            RelicSet::HunterOfGlacialForest => "104",
            RelicSet::ChampionOfStreetwiseBoxing => "105",
            RelicSet::GuardOfWutheringSnow => "106",
            RelicSet::FiresmithOfLavaForging => "107",
            RelicSet::GeniusOfBrilliantStars => "108",
            RelicSet::BandOfSizzlingThunder => "109",
            RelicSet::EagleOfTwilightLine => "110",
            RelicSet::ThiefOfShootingMeteor => "111",
            RelicSet::WastelanderOfBanditryDesert => "112",
            RelicSet::LongevousDisciple => "113",
            RelicSet::MessengerTraversingHackerspace => "114",
            RelicSet::TheAshblazingGrandDuke => "115",
            RelicSet::PrisonerInDeepConfinement => "116",
            RelicSet::SpaceSealingStation => "301",
            RelicSet::FleetOfTheAgeless => "302",
            RelicSet::PanCosmicCommercialEnterprise => "303",
            RelicSet::BelobogOfTheArchitects => "304",
            RelicSet::CelestialDifferentiator => "305",
            RelicSet::InertSalsotto => "306",
            RelicSet::TaliaKingdomOfBanditry => "307",
            RelicSet::SprightlyVonwacq => "308",
            RelicSet::RutilantArena => "309",
            RelicSet::BrokenKeel => "310",
            RelicSet::FirmamentFrontlineGlamoth => "311",
            RelicSet::PenaconyLandOfTheDreams => "312",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "Passerby of Wandering Cloud" => Some(RelicSet::PasserbyOfWanderingCloud),
            "Musketeer of Wild Wheat" => Some(RelicSet::MusketeerOfWildWheat),
            "Knight of Purity Palace" => Some(RelicSet::KnightOfPurityPalace),
            "Hunter of Glacial Forest" => Some(RelicSet::HunterOfGlacialForest),
            "Champion of Streetwise Boxing" => Some(RelicSet::ChampionOfStreetwiseBoxing),
            "Guard of Wuthering Snow" => Some(RelicSet::GuardOfWutheringSnow),
            "Firesmith of Lava-Forging" => Some(RelicSet::FiresmithOfLavaForging),
            "Genius of Brilliant Stars" => Some(RelicSet::GeniusOfBrilliantStars),
            "Band of Sizzling Thunder" => Some(RelicSet::BandOfSizzlingThunder),
            "Eagle of Twilight Line" => Some(RelicSet::EagleOfTwilightLine),
            "Thief of Shooting Meteor" => Some(RelicSet::ThiefOfShootingMeteor),
            "Wastelander of Banditry Desert" => Some(RelicSet::WastelanderOfBanditryDesert),
            "Longevous Disciple" => Some(RelicSet::LongevousDisciple),
            "Messenger Traversing Hackerspace" => Some(RelicSet::MessengerTraversingHackerspace),
            "The Ashblazing Grand Duke" => Some(RelicSet::TheAshblazingGrandDuke),
            "Prisoner in Deep Confinement" => Some(RelicSet::PrisonerInDeepConfinement),
            "Space Sealing Station" => Some(RelicSet::SpaceSealingStation),
            "Fleet of the Ageless" => Some(RelicSet::FleetOfTheAgeless),
            "Pan-Cosmic Commercial Enterprise" => Some(RelicSet::PanCosmicCommercialEnterprise),
            "Belobog of the Architects" => Some(RelicSet::BelobogOfTheArchitects),
            "Celestial Differentiator" => Some(RelicSet::CelestialDifferentiator),
            "Inert Salsotto" => Some(RelicSet::InertSalsotto),
            "Talia: Kingdom of Banditry" => Some(RelicSet::TaliaKingdomOfBanditry),
            "Sprightly Vonwacq" => Some(RelicSet::SprightlyVonwacq),
            "Rutilant Arena" => Some(RelicSet::RutilantArena),
            "Broken Keel" => Some(RelicSet::BrokenKeel),
            "Firmament Frontline: Glamoth" => Some(RelicSet::FirmamentFrontlineGlamoth),
            "Penacony, Land of the Dreams" => Some(RelicSet::PenaconyLandOfTheDreams),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub enum LightCone {
    Arrows,
    Cornucopia,
    CollapsingSky,
    Amber,
    Void,
    Chorus,
    DataBank,
    DartingArrow,
    FineFruit,
    ShatteredHome,
    Defense,
    Loop,
    MeshingCogs,
    Passkey,
    Adversarial,
    Multiplication,
    MutualDemise,
    Pioneering,
    HiddenShadow,
    Mediation,
    Sagacity,
    PostOpConversation,
    GoodNightAndSleepWell,
    DayOneOfMyNewLife,
    OnlySilenceRemains,
    MemoriesOfThePast,
    TheMolesWelcomeYou,
    TheBirthOfTheSelf,
    SharedFeeling,
    EyesOfThePrey,
    LandauSChoice,
    Swordplay,
    PlanetaryRendezvous,
    ASecretVow,
    MakeTheWorldClamor,
    PerfectTiming,
    ResolutionShinesAsPearlsOfSweat,
    TrendOfTheUniversalMarket,
    SubscribeForMore,
    DanceDanceDance,
    UnderTheBlueSky,
    GeniusesRepose,
    QuidProQuo,
    Fermata,
    WeAreWildfire,
    RiverFlowsInSpring,
    PastAndFuture,
    WoofWalkTime,
    TheSeriousnessOfBreakfast,
    WarmthShortensColdNights,
    WeWillMeetAgain,
    ThisIsMe,
    ReturnToDarkness,
    CarveTheMoonWeaveTheClouds,
    NowhereToRun,
    TodayIsAnotherPeacefulDay,
    BeforeTheTutorialMissionStarts,
    HeyOverHere,
    NightOnTheMilkyWay,
    InTheNight,
    SomethingIrreplaceable,
    ButTheBattleIsnTOver,
    InTheNameOfTheWorld,
    MomentOfVictory,
    PatienceIsAllYouNeed,
    IncessantRain,
    EchoesOfTheCoffin,
    TheUnreachableSide,
    BeforeDawn,
    SheAlreadyShutHerEyes,
    SleepLikeTheDead,
    TimeWaitsForNoOne,
    IShallBeMyOwnSword,
    BrighterThanTheSun,
    WorrisomeBlissful,
    NightOfFright,
    AnInstantBeforeAGaze,
    PastSelfInMirror,
    BaptismOfPureThought,
    OnTheFallOfAnAeon,
    CruisingInTheStellarSea,
    TextureOfMemories,
    SolitaryHealing,
}

impl LightCone {
    pub const COUNT: usize = 83;
    pub const fn to_id(self) -> &'static str {
        match self {
            LightCone::Arrows => "20000",
            LightCone::Cornucopia => "20001",
            LightCone::CollapsingSky => "20002",
            LightCone::Amber => "20003",
            LightCone::Void => "20004",
            LightCone::Chorus => "20005",
            LightCone::DataBank => "20006",
            LightCone::DartingArrow => "20007",
            LightCone::FineFruit => "20008",
            LightCone::ShatteredHome => "20009",
            LightCone::Defense => "20010",
            LightCone::Loop => "20011",
            LightCone::MeshingCogs => "20012",
            LightCone::Passkey => "20013",
            LightCone::Adversarial => "20014",
            LightCone::Multiplication => "20015",
            LightCone::MutualDemise => "20016",
            LightCone::Pioneering => "20017",
            LightCone::HiddenShadow => "20018",
            LightCone::Mediation => "20019",
            LightCone::Sagacity => "20020",
            LightCone::PostOpConversation => "21000",
            LightCone::GoodNightAndSleepWell => "21001",
            LightCone::DayOneOfMyNewLife => "21002",
            LightCone::OnlySilenceRemains => "21003",
            LightCone::MemoriesOfThePast => "21004",
            LightCone::TheMolesWelcomeYou => "21005",
            LightCone::TheBirthOfTheSelf => "21006",
            LightCone::SharedFeeling => "21007",
            LightCone::EyesOfThePrey => "21008",
            LightCone::LandauSChoice => "21009",
            LightCone::Swordplay => "21010",
            LightCone::PlanetaryRendezvous => "21011",
            LightCone::ASecretVow => "21012",
            LightCone::MakeTheWorldClamor => "21013",
            LightCone::PerfectTiming => "21014",
            LightCone::ResolutionShinesAsPearlsOfSweat => "21015",
            LightCone::TrendOfTheUniversalMarket => "21016",
            LightCone::SubscribeForMore => "21017",
            LightCone::DanceDanceDance => "21018",
            LightCone::UnderTheBlueSky => "21019",
            LightCone::GeniusesRepose => "21020",
            LightCone::QuidProQuo => "21021",
            LightCone::Fermata => "21022",
            LightCone::WeAreWildfire => "21023",
            LightCone::RiverFlowsInSpring => "21024",
            LightCone::PastAndFuture => "21025",
            LightCone::WoofWalkTime => "21026",
            LightCone::TheSeriousnessOfBreakfast => "21027",
            LightCone::WarmthShortensColdNights => "21028",
            LightCone::WeWillMeetAgain => "21029",
            LightCone::ThisIsMe => "21030",
            LightCone::ReturnToDarkness => "21031",
            LightCone::CarveTheMoonWeaveTheClouds => "21032",
            LightCone::NowhereToRun => "21033",
            LightCone::TodayIsAnotherPeacefulDay => "21034",
            LightCone::BeforeTheTutorialMissionStarts => "22000",
            LightCone::HeyOverHere => "22001",
            LightCone::NightOnTheMilkyWay => "23000",
            LightCone::InTheNight => "23001",
            LightCone::SomethingIrreplaceable => "23002",
            LightCone::ButTheBattleIsnTOver => "23003",
            LightCone::InTheNameOfTheWorld => "23004",
            LightCone::MomentOfVictory => "23005",
            LightCone::PatienceIsAllYouNeed => "23006",
            LightCone::IncessantRain => "23007",
            LightCone::EchoesOfTheCoffin => "23008",
            LightCone::TheUnreachableSide => "23009",
            LightCone::BeforeDawn => "23010",
            LightCone::SheAlreadyShutHerEyes => "23011",
            LightCone::SleepLikeTheDead => "23012",
            LightCone::TimeWaitsForNoOne => "23013",
            LightCone::IShallBeMyOwnSword => "23014",
            LightCone::BrighterThanTheSun => "23015",
            LightCone::WorrisomeBlissful => "23016",
            LightCone::NightOfFright => "23017",
            LightCone::AnInstantBeforeAGaze => "23018",
            LightCone::PastSelfInMirror => "23019",
            LightCone::BaptismOfPureThought => "23020",
            LightCone::OnTheFallOfAnAeon => "24000",
            LightCone::CruisingInTheStellarSea => "24001",
            LightCone::TextureOfMemories => "24002",
            LightCone::SolitaryHealing => "24003",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "Arrows" => Some(LightCone::Arrows),
            "Cornucopia" => Some(LightCone::Cornucopia),
            "Collapsing Sky" => Some(LightCone::CollapsingSky),
            "Amber" => Some(LightCone::Amber),
            "Void" => Some(LightCone::Void),
            "Chorus" => Some(LightCone::Chorus),
            "Data Bank" => Some(LightCone::DataBank),
            "Darting Arrow" => Some(LightCone::DartingArrow),
            "Fine Fruit" => Some(LightCone::FineFruit),
            "Shattered Home" => Some(LightCone::ShatteredHome),
            "Defense" => Some(LightCone::Defense),
            "Loop" => Some(LightCone::Loop),
            "Meshing Cogs" => Some(LightCone::MeshingCogs),
            "Passkey" => Some(LightCone::Passkey),
            "Adversarial" => Some(LightCone::Adversarial),
            "Multiplication" => Some(LightCone::Multiplication),
            "Mutual Demise" => Some(LightCone::MutualDemise),
            "Pioneering" => Some(LightCone::Pioneering),
            "Hidden Shadow" => Some(LightCone::HiddenShadow),
            "Mediation" => Some(LightCone::Mediation),
            "Sagacity" => Some(LightCone::Sagacity),
            "Post-Op Conversation" => Some(LightCone::PostOpConversation),
            "Good Night and Sleep Well" => Some(LightCone::GoodNightAndSleepWell),
            "Day One of My New Life" => Some(LightCone::DayOneOfMyNewLife),
            "Only Silence Remains" => Some(LightCone::OnlySilenceRemains),
            "Memories of the Past" => Some(LightCone::MemoriesOfThePast),
            "The Moles Welcome You" => Some(LightCone::TheMolesWelcomeYou),
            "The Birth of the Self" => Some(LightCone::TheBirthOfTheSelf),
            "Shared Feeling" => Some(LightCone::SharedFeeling),
            "Eyes of the Prey" => Some(LightCone::EyesOfThePrey),
            "Landau's Choice" => Some(LightCone::LandauSChoice),
            "Swordplay" => Some(LightCone::Swordplay),
            "Planetary Rendezvous" => Some(LightCone::PlanetaryRendezvous),
            "A Secret Vow" => Some(LightCone::ASecretVow),
            "Make the World Clamor" => Some(LightCone::MakeTheWorldClamor),
            "Perfect Timing" => Some(LightCone::PerfectTiming),
            "Resolution Shines As Pearls of Sweat" => Some(LightCone::ResolutionShinesAsPearlsOfSweat),
            "Trend of the Universal Market" => Some(LightCone::TrendOfTheUniversalMarket),
            "Subscribe for More!" => Some(LightCone::SubscribeForMore),
            "Dance! Dance! Dance!" => Some(LightCone::DanceDanceDance),
            "Under the Blue Sky" => Some(LightCone::UnderTheBlueSky),
            "Geniuses' Repose" => Some(LightCone::GeniusesRepose),
            "Quid Pro Quo" => Some(LightCone::QuidProQuo),
            "Fermata" => Some(LightCone::Fermata),
            "We Are Wildfire" => Some(LightCone::WeAreWildfire),
            "River Flows in Spring" => Some(LightCone::RiverFlowsInSpring),
            "Past and Future" => Some(LightCone::PastAndFuture),
            "Woof! Walk Time!" => Some(LightCone::WoofWalkTime),
            "The Seriousness of Breakfast" => Some(LightCone::TheSeriousnessOfBreakfast),
            "Warmth Shortens Cold Nights" => Some(LightCone::WarmthShortensColdNights),
            "We Will Meet Again" => Some(LightCone::WeWillMeetAgain),
            "This Is Me!" => Some(LightCone::ThisIsMe),
            "Return to Darkness" => Some(LightCone::ReturnToDarkness),
            "Carve the Moon, Weave the Clouds" => Some(LightCone::CarveTheMoonWeaveTheClouds),
            "Nowhere to Run" => Some(LightCone::NowhereToRun),
            "Today Is Another Peaceful Day" => Some(LightCone::TodayIsAnotherPeacefulDay),
            "Before the Tutorial Mission Starts" => Some(LightCone::BeforeTheTutorialMissionStarts),
            "Hey, Over Here" => Some(LightCone::HeyOverHere),
            "Night on the Milky Way" => Some(LightCone::NightOnTheMilkyWay),
            "In the Night" => Some(LightCone::InTheNight),
            "Something Irreplaceable" => Some(LightCone::SomethingIrreplaceable),
            "But the Battle Isn't Over" => Some(LightCone::ButTheBattleIsnTOver),
            "In the Name of the World" => Some(LightCone::InTheNameOfTheWorld),
            "Moment of Victory" => Some(LightCone::MomentOfVictory),
            "Patience Is All You Need" => Some(LightCone::PatienceIsAllYouNeed),
            "Incessant Rain" => Some(LightCone::IncessantRain),
            "Echoes of the Coffin" => Some(LightCone::EchoesOfTheCoffin),
            "The Unreachable Side" => Some(LightCone::TheUnreachableSide),
            "Before Dawn" => Some(LightCone::BeforeDawn),
            "She Already Shut Her Eyes" => Some(LightCone::SheAlreadyShutHerEyes),
            "Sleep Like the Dead" => Some(LightCone::SleepLikeTheDead),
            "Time Waits for No One" => Some(LightCone::TimeWaitsForNoOne),
            "I Shall Be My Own Sword" => Some(LightCone::IShallBeMyOwnSword),
            "Brighter Than the Sun" => Some(LightCone::BrighterThanTheSun),
            "Worrisome, Blissful" => Some(LightCone::WorrisomeBlissful),
            "Night of Fright" => Some(LightCone::NightOfFright),
            "An Instant Before A Gaze" => Some(LightCone::AnInstantBeforeAGaze),
            "Past Self in Mirror" => Some(LightCone::PastSelfInMirror),
            "Baptism of Pure Thought" => Some(LightCone::BaptismOfPureThought),
            "On the Fall of an Aeon" => Some(LightCone::OnTheFallOfAnAeon),
            "Cruising in the Stellar Sea" => Some(LightCone::CruisingInTheStellarSea),
            "Texture of Memories" => Some(LightCone::TextureOfMemories),
            "Solitary Healing" => Some(LightCone::SolitaryHealing),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub enum Character {
    March7th,
    DanHeng,
    Himeko,
    Welt,
    Kafka,
    SilverWolf,
    Arlan,
    Asta,
    Herta,
    Bronya,
    Seele,
    Serval,
    Gepard,
    Natasha,
    Pela,
    Clara,
    Sampo,
    Hook,
    Lynx,
    Luka,
    TopazAndNumby,
    Qingque,
    Tingyun,
    Luocha,
    JingYuan,
    Blade,
    Sushang,
    Yukong,
    FuXuan,
    Yanqing,
    Guinaifen,
    Bailu,
    Jingliu,
    DanHengImbibitorLunae,
    Xueyi,
    Hanya,
    Huohuo,
    Argenti,
    RuanMei,
    DrRatio,
    PhysicalTrailblazerM,
    PhysicalTrailblazerF,
    FireTrailblazerM,
    FireTrailblazerF,
}

impl Character {
    pub const COUNT: usize = 44;
    pub const fn to_id(self) -> &'static str {
        match self {
            Character::March7th => "1001",
            Character::DanHeng => "1002",
            Character::Himeko => "1003",
            Character::Welt => "1004",
            Character::Kafka => "1005",
            Character::SilverWolf => "1006",
            Character::Arlan => "1008",
            Character::Asta => "1009",
            Character::Herta => "1013",
            Character::Bronya => "1101",
            Character::Seele => "1102",
            Character::Serval => "1103",
            Character::Gepard => "1104",
            Character::Natasha => "1105",
            Character::Pela => "1106",
            Character::Clara => "1107",
            Character::Sampo => "1108",
            Character::Hook => "1109",
            Character::Lynx => "1110",
            Character::Luka => "1111",
            Character::TopazAndNumby => "1112",
            Character::Qingque => "1201",
            Character::Tingyun => "1202",
            Character::Luocha => "1203",
            Character::JingYuan => "1204",
            Character::Blade => "1205",
            Character::Sushang => "1206",
            Character::Yukong => "1207",
            Character::FuXuan => "1208",
            Character::Yanqing => "1209",
            Character::Guinaifen => "1210",
            Character::Bailu => "1211",
            Character::Jingliu => "1212",
            Character::DanHengImbibitorLunae => "1213",
            Character::Xueyi => "1214",
            Character::Hanya => "1215",
            Character::Huohuo => "1217",
            Character::Argenti => "1302",
            Character::RuanMei => "1303",
            Character::DrRatio => "1305",
            Character::PhysicalTrailblazerM => "8001",
            Character::PhysicalTrailblazerF => "8002",
            Character::FireTrailblazerM => "8003",
            Character::FireTrailblazerF => "8004",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "March 7th" => Some(Character::March7th),
            "Dan Heng" => Some(Character::DanHeng),
            "Himeko" => Some(Character::Himeko),
            "Welt" => Some(Character::Welt),
            "Kafka" => Some(Character::Kafka),
            "Silver Wolf" => Some(Character::SilverWolf),
            "Arlan" => Some(Character::Arlan),
            "Asta" => Some(Character::Asta),
            "Herta" => Some(Character::Herta),
            "Bronya" => Some(Character::Bronya),
            "Seele" => Some(Character::Seele),
            "Serval" => Some(Character::Serval),
            "Gepard" => Some(Character::Gepard),
            "Natasha" => Some(Character::Natasha),
            "Pela" => Some(Character::Pela),
            "Clara" => Some(Character::Clara),
            "Sampo" => Some(Character::Sampo),
            "Hook" => Some(Character::Hook),
            "Lynx" => Some(Character::Lynx),
            "Luka" => Some(Character::Luka),
            "Topaz & Numby" => Some(Character::TopazAndNumby),
            "Qingque" => Some(Character::Qingque),
            "Tingyun" => Some(Character::Tingyun),
            "Luocha" => Some(Character::Luocha),
            "Jing Yuan" => Some(Character::JingYuan),
            "Blade" => Some(Character::Blade),
            "Sushang" => Some(Character::Sushang),
            "Yukong" => Some(Character::Yukong),
            "Fu Xuan" => Some(Character::FuXuan),
            "Yanqing" => Some(Character::Yanqing),
            "Guinaifen" => Some(Character::Guinaifen),
            "Bailu" => Some(Character::Bailu),
            "Jingliu" => Some(Character::Jingliu),
            "Dan Heng • Imbibitor Lunae" => Some(Character::DanHengImbibitorLunae),
            "Xueyi" => Some(Character::Xueyi),
            "Hanya" => Some(Character::Hanya),
            "Huohuo" => Some(Character::Huohuo),
            "Argenti" => Some(Character::Argenti),
            "Ruan Mei" => Some(Character::RuanMei),
            "Dr. Ratio" => Some(Character::DrRatio),
            "{NICKNAME}" => Some(Character::PhysicalTrailblazerM),
            "{NICKNAME}" => Some(Character::PhysicalTrailblazerF),
            "{NICKNAME}" => Some(Character::FireTrailblazerM),
            "{NICKNAME}" => Some(Character::FireTrailblazerF),
            _ => None,
        }
    }
}