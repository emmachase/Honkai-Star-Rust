import { Character, CharacterConfig, EffectPropertyType, IShallBeMyOwnSwordConfig, JingliuConfig, Relic, RelicSlot, ResolvedCalculatorResult, SortResultsSerde, commands } from "@/bindings.gen";
import { OptimizerTable } from "@/components/domain/optimizer-table";
import { Button } from "@/components/ui/button";
import { Combobox } from "@/components/ui/combobox";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Column, Row } from "@/components/util/flex";
import { CharacterKitComponent, CharacterKitMap, Characters } from "@/kits/characters";
import { JingliuKit } from "@/kits/characters/jingliu";
import { IShallBeMyOwnSwordKit } from "@/kits/lightcones/i-shall-be-my-own-sword";
import { useCalcs, useData } from "@/store";
import { cn } from "@/utils";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { PropsWithChildren, Suspense, startTransition, useEffect, useMemo, useState } from "react";

export const Route = createFileRoute("/")({
    component: Index,
});

function Card({ children, className, ...props }: PropsWithChildren<React.HTMLAttributes<HTMLDivElement>>) {
    return (
        <div className={cn("border p-4 rounded-md bg-card w-[260px]", className)} {...props}>
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

    const running = useCalcs(c => c.running);

    return (
        <Card>
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

                <Button size="sm" onClick={triggerSearch}>{running ? "Cancel Search" : "Search"}</Button>
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
        <Card>
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

function CharacterPreview(props: { character: Character }) {
    const src = useSuspenseQuery({
        queryKey: ["character_preview", props.character],
        queryFn: () => commands.getCharPic(props.character),
    })

    return <img src={"/hsr/" + src.data} className="w-full h-[300px] object-cover"/>
}

function CharacterPreviewCard(props: { character: Character }) {
    return (
        <Card className="p-0 bg-transparent">
            <Suspense fallback="Loading...">
                <CharacterPreview character={props.character}/>
            </Suspense>
        </Card>
    )
}

const characterState = {
    level: 80,
    eidolon: 0,
    ascension: 6,
    skills: {
        basic: 5 - 1,
        skill: 10 - 1,
        ult: 10 - 1,
        talent: 10 - 1,
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

const lcState = {
    ascension: 6,
    level: 80,
    superimposition: 1 - 1,
}

function CharacterKitCard<C extends Characters>(props: { character: C, onChange?: (value: CharacterConfig) => void }) {
    type Config = (typeof CharacterKitMap)[C]["defaultConfig"];
    const characterKitShit = CharacterKitMap[props.character];
    const MyComponent = characterKitShit.component as CharacterKitComponent<Config>;
    const [kit, setKit] = useState<Config>(characterKitShit.defaultConfig);
    useEffect(() => {
        if (props.onChange) {
            props.onChange(characterKitShit.wrapConfig(kit));
        }
    }, [kit]);

    return (
        <Card>
            <Suspense fallback="Loading...">
                <MyComponent
                    characterState={characterState}
                    value={kit}
                    onChange={setKit}
                />
            </Suspense>
        </Card>
    )
}

function Index() {
    const [character, setCharacter] = useState<Characters>(Characters.Jingliu);

    // const characterKitShit = CharacterKitMap[character];
    const [kit, setKit] = useState<CharacterConfig>();

    const [lcKit, setLcKit] = useState<IShallBeMyOwnSwordConfig>({
        eclipse_stacks: 3,
        max_stack_def_pen: true,
    });

    const [filters, setFilters] = useState<((r: Relic) => boolean)[]>([])

    const allRelics = useData(d => d.relics);
    const filteredRelics = useMemo(() => {
        return allRelics.filter(r => filters.every(f => f(r)));
    }, [allRelics, filters]);

    const [result, setResult] = useCalcs(c => [c.sortResults, c.setSortResults]); // useState<SortResultsSerde>();
    const [running, setRunning] = useCalcs(c => [c.running, c.setRunning]); // TODO
    const triggerSearch = async () => {
        if (running) {
            await commands.stopPranking();
        } else {
            if (!kit) {
                console.error("No kit");
                return;
            }

            setRunning(true);
            setResult(await commands.prankHimJohn(
                filteredRelics,
                // { Jingliu: kit },
                kit,
                characterState,
                { IShallBeMyOwnSword: lcKit },
                lcState,
                {
                    count: 1,
                    level: 95,

                    resistance: 0.2,
                    elemental_weakness: true,
                    weakness_broken: false,
                    debuff_count: 3,
                },
            ));
            setRunning(false);
        }
    };

    return (
        <Column className="min-w-0">
            <h3>Welcome Home!</h3>
            <Combobox
                value={character}
                onChange={(c) => startTransition(() => {
                    setCharacter(c)
                    // setKit(CharacterKitMap[c].defaultConfig)
                })}
                deselectable={false}
                options={[
                    { value: Characters.Jingliu, label: "Jingliu" },
                    // { value: Characters.Xueyi, label: "Xueyi" },
                    { value: Characters.Sparkle, label: "Sparkle" },
                ]}
            />

            <Row className="flex-wrap gap-2 justify-center">
                {/* <Card className="p-0 bg-transparent">
                    <img src="/hsr/image/character_preview/1212.png" className="w-full h-[300px] object-cover"/>
                </Card> */}
                <CharacterPreviewCard character={character as Character} />

                {/* <Card>
                    <Suspense fallback="Loading...">
                        <characterKitShit.component
                            characterState={characterState}
                            value={kit as any}
                            onChange={setKit}
                        />
                    </Suspense>
                </Card> */}

                <CharacterKitCard key={character} character={character} onChange={setKit} />

                <Card>
                    <Suspense fallback="Loading...">
                        <IShallBeMyOwnSwordKit
                            lightConeState={lcState}
                            value={lcKit}
                            onChange={setLcKit}
                        />
                    </Suspense>
                </Card>

                <PermutationCard
                    allRelics={allRelics}
                    filteredRelics={filteredRelics}
                    triggerSearch={triggerSearch}
                />

                <MainStatFilterCard onChange={fs => setFilters(fs)}/>
            </Row>

            {/* {JSON.stringify(result)} */}

            <div className="w-full relative">
                <ScrollArea
                    className="w-full"
                    scrollbar={<ScrollBar orientation="horizontal" />}
                >
                    <OptimizerTable className="w-full"
                        data={result}
                    />
                </ScrollArea>
            </div>
        </Column>
    );
}
