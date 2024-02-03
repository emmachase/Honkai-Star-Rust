import { PropsWithChildren, useState } from "react";
// import { Button, Flex, Grid, Heading, TextField, View } from "@adobe/react-spectrum";
import { EffectPropertyType, commands } from "./bindings.gen";
import { Button } from "./components/ui/button";
import { Column } from "./components/util/flex";
// import { SideNav, SideNavButton } from "./components/SideNav";
// import Beaker from "@spectrum-icons/workflow/Beaker";
// import PeopleGroup from "@spectrum-icons/workflow/PeopleGroup";
// import { MultiComboBox } from "./components/MultiComboBox";

// <MultiComboBox<EffectPropertyType> label="Chest Filter" options={[
//     {label: "CRIT Rate", value: "CriticalChanceBase"},
//     {label: "CRIT DMG", value: "CriticalDamageBase"},
//     {label: "ATK%", value: "AttackAddedRatio"},
//     {label: "DEF%", value: "DefenceAddedRatio"},
//     {label: "HP%", value: "HPAddedRatio"},
//     {label: "Effect Hit Rate", value: "StatusProbabilityBase"},
//     {label: "Outgoing Healing Boost", value: "HealRatioBase"},
//     ]}/>
//     <MultiComboBox<EffectPropertyType> label="Feet Pics" options={[
//     {label: "ATK%", value: "AttackAddedRatio"},
//     {label: "DEF%", value: "DefenceAddedRatio"},
//     {label: "HP%", value: "HPAddedRatio"},
//     {label: "SPD", value: "SpeedDelta"},
//     ]}/>
//     <MultiComboBox<EffectPropertyType> label="Planar Sphere Filter" options={[
//     {label: "ATK%", value: "AttackAddedRatio"},
//     {label: "DEF%", value: "DefenceAddedRatio"},
//     {label: "HP%", value: "HPAddedRatio"},
//     {label: "Physical DMG Boost", value: "PhysicalAddedRatio"},
//     {label: "Fire DMG Boost", value: "FireAddedRatio"},
//     {label: "Ice DMG Boost", value: "IceAddedRatio"},
//     {label: "Thunder DMG Boost", value: "ThunderAddedRatio"},
//     {label: "Wind DMG Boost", value: "WindAddedRatio"},
//     {label: "Quantum DMG Boost", value: "QuantumAddedRatio"},
//     {label: "Imaginary DMG Boost", value: "ImaginaryAddedRatio"},
//     ]}/>
//     <MultiComboBox<EffectPropertyType> label="Link Rope Filter" options={[
//     {label: "ATK%", value: "AttackAddedRatio"},
//     {label: "DEF%", value: "DefenceAddedRatio"},
//     {label: "HP%", value: "HPAddedRatio"},
//     {label: "Break Effect", value: "BreakDamageAddedRatioBase"},
//     {label: "Energy Regeneration Rate", value: "SPRatioBase"},
//     ]}/>



function App() {
    const [greetMsg, setGreetMsg] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        // setGreetMsg(await invoke("greet", { name }));
        setGreetMsg(
            await commands.prankHimJohn(
                {
                    Jingliu: {
                        e1_crit_dmg: true,
                        e2_skill_buff: true,
                        enhanced_state: true,
                        hp_drain_pct: 1.0,
                    },
                },
                {
                    level: 80,
                    eidolon: 0,
                    ascension: 6,
                    skills: {
                        basic: 7 - 1,
                        skill: 12 - 1,
                        ult: 12 - 1,
                        talent: 12 - 1,
                    },
                    traces: {
                        ability_1: true,
                        ability_2: true,
                        ability_3: true,
                        stat_1: true,
                        stat_2: true,
                        stat_3: true,
                        stat_4: true,
                        stat_5: true,
                        stat_6: true,
                        stat_7: true,
                        stat_8: true,
                        stat_9: true,
                        stat_10: true,
                    },
                },
                {
                    IShallBeMyOwnSword: {
                        eclipse_stacks: 3,
                        max_stack_def_pen: true,
                    },
                },
                {
                    ascension: 6,
                    level: 80,
                    superimposition: 5 - 1,
                },
                {
                    count: 1,
                    level: 95,

                    resistance: 0.2,
                    elemental_weakness: true,
                    weakness_broken: false,
                }
            )
        );
    }

    return (
        <div className="App">


                <div>
                    <Button onClick={greet} size="sm" variant="default">
                        Greet
                    </Button>
                    <p>{greetMsg}</p>
                </div>
        </div>
    );
}

export default App;
