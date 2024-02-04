import { EffectPropertyType, JingliuConfig, Relic, RelicSlot, ResolvedCalculatorResult, SortResultsSerde, commands } from "@/bindings.gen";
import { OptimizerTable } from "@/components/domain/optimizer-table";
import { Button } from "@/components/ui/button";
import { Combobox } from "@/components/ui/combobox";
import { Column, Row } from "@/components/util/flex";
import { JingliuKit } from "@/kits/characters/jingliu";
import { useData } from "@/store";
import { cn } from "@/utils";
import { createFileRoute } from "@tanstack/react-router";
import { PropsWithChildren, useEffect, useMemo, useState } from "react";

export const Route = createFileRoute("/")({
    component: Index,
});

enum Characters {
    Jingliu = "jingliu",
    Xueyi = "xueyi",
    Sparkle = "sparkle",
}

function Card({ children, className, ...props }: PropsWithChildren<React.HTMLAttributes<HTMLDivElement>>) {
    return (
        <div className={cn("border p-4 rounded-md bg-card", className)} {...props}>
            {children}
        </div>
    );
}

function countRelics(relics: Relic[]) {
    return {
        head: relics.filter(r => r.slot === "Head").length,
        hands: relics.filter(r => r.slot === "Hands").length,
        chest: relics.filter(r => r.slot === "Chest").length,
        feet: relics.filter(r => r.slot === "Feet").length,
        sphere: relics.filter(r => r.slot === "PlanarSphere").length,
        linkRope: relics.filter(r => r.slot === "LinkRope").length,
    }
}

function permutations(items: ReturnType<typeof countRelics>) {
    return items.head * items.hands * items.chest * items.feet * items.sphere * items.linkRope;
}

const Dash = () => <div className="flex-1 h-0 border-b border-dashed" />

function CountString(props: {
    filtered: number,
    total: number,
}) {
    if (props.total === 0) {
        return (
            <>0 / 0 (0%)</>
        )
    }

    return (
        <>
            <Dash />
            {props.filtered} / {props.total} ({(props.filtered / props.total * 100).toFixed(2)}%)
        </>
    )
}

function PermutationCard({ allRelics, filteredRelics, triggerSearch }: {
    allRelics: Relic[],
    filteredRelics: Relic[],
    triggerSearch: () => void,
}) {
    const [filteredCounts, totalCounts] = useMemo(() => {
        const filteredCounts = countRelics(filteredRelics);
        const totalCounts = countRelics(allRelics);

        return [filteredCounts, totalCounts];
    }, [filteredRelics, allRelics]);

    return (
        <Card className="w-[300px]">
            <Column>
                <div>
                    <Row className="items-center gap-2">Head <CountString filtered={filteredCounts.head} total={totalCounts.head} /></Row>
                    <Row className="items-center gap-2">Hands <CountString filtered={filteredCounts.hands} total={totalCounts.hands} /></Row>
                    <Row className="items-center gap-2">Chest <CountString filtered={filteredCounts.chest} total={totalCounts.chest} /></Row>
                    <Row className="items-center gap-2">Feet <CountString filtered={filteredCounts.feet} total={totalCounts.feet} /></Row>
                    <Row className="items-center gap-2">Rope <CountString filtered={filteredCounts.linkRope} total={totalCounts.linkRope} /></Row>
                    <Row className="items-center gap-2">Sphere <CountString filtered={filteredCounts.sphere} total={totalCounts.sphere} /></Row>
                </div>
                <div>
                    <Row className="items-center gap-2">Perms <Dash /> {permutations(filteredCounts).toLocaleString()}</Row>
                </div>

                <Button size="sm" onClick={triggerSearch}>Search</Button>
            </Column>
        </Card>
    )
}

function makeFilter(slot: RelicSlot, filters: EffectPropertyType[]) {
    return (r: Relic) => {
        if (r.slot !== slot) {
            return true; // Ignore
        }

        if (filters.length === 0) {
            return true;
        }

        return filters.includes(r.main_stat[0]);
    }
}

// <MultiComboBox<EffectPropertyType> label="Chest Filter" options={[
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

function MainStatFilterCard(props: {
    onChange?: (filters: ((r: Relic) => boolean)[]) => void,
}) {
    const [chestFilter, setChestFilter]       = useState<EffectPropertyType[]>([]);
    const [feetFilter, setFeetFilter]         = useState<EffectPropertyType[]>([]);
    const [sphereFilter, setSphereFilter]     = useState<EffectPropertyType[]>([]);
    const [linkRopeFilter, setLinkRopeFilter] = useState<EffectPropertyType[]>([]);

    useEffect(() => {
        if (props.onChange) {
            props.onChange([
                makeFilter("Chest", chestFilter),
                makeFilter("Feet", feetFilter),
                makeFilter("PlanarSphere", sphereFilter),
                makeFilter("LinkRope", linkRopeFilter),
            ])
        }
    }, [chestFilter, feetFilter, sphereFilter, linkRopeFilter])

    return (
        <Card className="w-[300px]">
            <Column className="gap-1">
                <Combobox<EffectPropertyType> multiple className="w-full"
                    placeholder="Chest Main Stat"
                    options={[
                        { label: "CRIT Rate", value: "CriticalChanceBase" },
                        { label: "CRIT DMG", value: "CriticalDamageBase" },
                        { label: "ATK%", value: "AttackAddedRatio" },
                        { label: "DEF%", value: "DefenceAddedRatio" },
                        { label: "HP%", value: "HPAddedRatio" },
                        { label: "Effect Hit Rate", value: "StatusProbabilityBase" },
                        { label: "Outgoing Healing Boost", value: "HealRatioBase" },
                    ]}
                    value={chestFilter}
                    onChange={values => setChestFilter(values)}
                />

                <Combobox<EffectPropertyType> multiple className="w-full"
                    placeholder="Feet Main Stat"
                    options={[
                        { label: "ATK%", value: "AttackAddedRatio" },
                        { label: "DEF%", value: "DefenceAddedRatio" },
                        { label: "HP%", value: "HPAddedRatio" },
                        { label: "SPD", value: "SpeedDelta" },
                    ]}
                    value={feetFilter}
                    onChange={values => setFeetFilter(values)}
                />

                <Combobox<EffectPropertyType> multiple className="w-full"
                    placeholder="Planar Sphere Main Stat"
                    options={[
                        { label: "ATK%", value: "AttackAddedRatio" },
                        { label: "DEF%", value: "DefenceAddedRatio" },
                        { label: "HP%", value: "HPAddedRatio" },
                        { label: "Physical DMG Boost", value: "PhysicalAddedRatio" },
                        { label: "Fire DMG Boost", value: "FireAddedRatio" },
                        { label: "Ice DMG Boost", value: "IceAddedRatio" },
                        { label: "Thunder DMG Boost", value: "ThunderAddedRatio" },
                        { label: "Wind DMG Boost", value: "WindAddedRatio" },
                        { label: "Quantum DMG Boost", value: "QuantumAddedRatio" },
                        { label: "Imaginary DMG Boost", value: "ImaginaryAddedRatio" },
                    ]}
                    value={sphereFilter}
                    onChange={values => setSphereFilter(values)}
                />

                <Combobox<EffectPropertyType> multiple className="w-full"
                    placeholder="Link Rope Main Stat"
                    options={[
                        { label: "ATK%", value: "AttackAddedRatio" },
                        { label: "DEF%", value: "DefenceAddedRatio" },
                        { label: "HP%", value: "HPAddedRatio" },
                        { label: "Break Effect", value: "BreakDamageAddedRatioBase" },
                        { label: "Energy Regeneration Rate", value: "SPRatioBase" },
                    ]}
                    value={linkRopeFilter}
                    onChange={values => setLinkRopeFilter(values)}
                />
            </Column>
        </Card>
    )
}

const characterState = {
    level: 80,
    eidolon: 6,
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
}

function Index() {
    const [character, setCharacter] = useState<string>(Characters.Jingliu);

    const [kit, setKit] = useState<JingliuConfig>({
        enhanced_state: true,
        e1_crit_dmg: true,
        e2_skill_buff: true,
        hp_drain_pct: 1.0,
    });

    const [filters, setFilters] = useState<((r: Relic) => boolean)[]>([])

    const allRelics = useData(d => d.relics);
    const filteredRelics = useMemo(() => {
        return allRelics.filter(r => filters.every(f => f(r)));
    }, [allRelics, filters]);

    const [result, setResult] = useState<SortResultsSerde>();
    const triggerSearch = async () => {
        setResult(await commands.prankHimJohn(
            filteredRelics,
            { Jingliu: kit },
            characterState,
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
            },
        ));
    };

    return (
        <div>
            <h3>Welcome Home!</h3>
            <Combobox
                value={character}
                onChange={setCharacter}
                deselectable={false}
                options={[
                    { value: Characters.Jingliu, label: "Jingliu" },
                    { value: Characters.Xueyi, label: "Xueyi" },
                    { value: Characters.Sparkle, label: "Sparkle" },
                ]}
            />

            <div className="flex flex-wrap">
                <Card className="w-[300px]">
                    <JingliuKit
                        characterState={characterState}
                        value={kit}
                        onChange={(value) => {
                            console.log(value);
                            setKit(value);
                        }}
                    />
                </Card>

                <PermutationCard
                    allRelics={allRelics}
                    filteredRelics={filteredRelics}
                    triggerSearch={triggerSearch}
                />

                <MainStatFilterCard onChange={fs => setFilters(fs)}/>
            </div>

            {/* {result} */}
            <OptimizerTable
                data={result}
            />
        </div>
    );
}
