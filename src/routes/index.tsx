import { Character, CharacterConfig, EffectPropertyType, IShallBeMyOwnSwordConfig, JingliuConfig, LightCone, LightConeConfig, Relic, RelicSlot, ResolvedCalculatorResult, SortResultsSerde, StatColumnType, StatFilter, StatFilterType, commands } from "@/bindings.gen";
import { OptimizerTable } from "@/components/domain/optimizer-table";
import { Button } from "@/components/ui/button";
import { ButtonGroup } from "@/components/ui/button-group";
import { Combobox } from "@/components/ui/combobox";
import { Header } from "@/components/ui/header";
import { Input } from "@/components/ui/input";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Column, Row } from "@/components/util/flex";
import { CharacterKitComponent, CharacterKitMap, Characters } from "@/kits/characters";
import { LightConeKitComponent, LightConeKitMap, LightCones } from "@/kits/light-cones";
import { IShallBeMyOwnSwordKit } from "@/kits/lightcones/i-shall-be-my-own-sword";
import { useCalcs, useCharacters, useRelics, useSession } from "@/store";
import { cn } from "@/utils";
import { useForm } from "@/utils/form";
import { UnionToIntersection } from "@/utils/magic";
import { findAndMap } from "@/utils/misc";
import { useQuery, useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { PropsWithChildren, Suspense, startTransition, useDeferredValue, useEffect, useMemo, useState } from "react";

export const Route = createFileRoute("/")({
    component: Index,
});

function CardTitle(props: PropsWithChildren<React.HTMLAttributes<HTMLElement>>) {
    return (
        <header className="font-bold text-xs" {...props} />
    )
}

function Card({ children, className, ...props }: PropsWithChildren<React.HTMLAttributes<HTMLDivElement>>) {
    return (
        <div className={cn("border p-4 rounded-md bg-card w-[280px]", className)} {...props}>
            <Column>
                {children}
            </Column>
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
        queryFn: () => commands.getCharPreview(props.character),
    })

    return <img src={"/hsr/" + src.data} className="w-full h-[320px] object-cover"/>
}

function LightConeIcon(props: { lightCone: LightCone, className?: string }) {
    const src = useSuspenseQuery({
        queryKey: ["light_cone_icon", props.lightCone],
        queryFn: () => commands.getLcIcon(props.lightCone),
    })

    return <img src={"/hsr/" + src.data} className={cn("absolute right-2 bottom-2 scale-75 origin-bottom-right", props.className)}/>
}

function CharacterPreviewCard(props: { character: Character, lightCone?: LightCone }) {
    const character = useDeferredValue(props.character);
    const lightCone = useDeferredValue(props.lightCone);

    return (
        <Card className="p-0 bg-transparent relative h-[320px]">
            <Suspense fallback="">
                <CharacterPreview character={character}/>
                {lightCone && <>
                    <LightConeIcon lightCone={lightCone} className="blur-md"/>
                    <LightConeIcon lightCone={lightCone}/>
                </>}
            </Suspense>
        </Card>
    )
}



// const lcState = {
//     ascension: 6,
//     level: 80,
//     superimposition: 1 - 1,
// }

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

    const characterState = useCharacters(s => s.getCharacter(props.character).state) // s.characters[props.character]?.state) ?? defaultCharacterState;

    return (
        <Card>
            <CardTitle>Character Config</CardTitle>
            <Suspense fallback="">
                <MyComponent
                    characterState={characterState}
                    value={kit}
                    onChange={setKit}
                />
            </Suspense>
        </Card>
    )
}

function LightConeKitCard<C extends LightCones>(props: { character: Characters, lightCone: C | undefined, onChange?: (value: LightConeConfig) => void }) {
    if (props.lightCone === undefined) {
        return <Card>
            <CardTitle>Light Cone Config</CardTitle>
            No Light Cone Selected
        </Card>
    }

    type Config = (typeof LightConeKitMap)[C]["defaultConfig"];
    const lightConeKitShit = LightConeKitMap[props.lightCone];
    const MyComponent = lightConeKitShit.component as LightConeKitComponent<Config>;
    const [kit, setKit] = useState<Config>(lightConeKitShit.defaultConfig);
    useEffect(() => {
        if (props.onChange) {
            props.onChange(lightConeKitShit.wrapConfig(kit));
        }
    }, [kit]);

    const lightConeState = useCharacters(s => s.getCharacter(props.character).lightCone?.[1]) // s.lightCones[props.lightCone]?.state) ?? defaultLightConeState;

    return (
        <Card>
            <CardTitle>Light Cone Config</CardTitle>
            { lightConeState &&
                <Suspense fallback="">
                    <MyComponent
                        lightConeState={lightConeState}
                        value={kit}
                        onChange={setKit}
                    />
                </Suspense>
            }
        </Card>
    )
}

// 0-20  A0
// 20-30 A1
// 30-40 A2
// 40-50 A3
// 50-60 A4
// 60-70 A5
// 70-80 A6
function *generateLevels() {
    function makeValue(i: number, ascension: number) {
        return { value: `${i};${ascension}`, label: `Level ${i}, A${ascension}` };
    }

    for (let i = 1; i <= 80; i++) {
        const ascension = Math.max(0, Math.floor((i - 11) / 10));
        yield makeValue(i, ascension);

        if (i % 10 === 0 && i > 10 && i < 80) {
            yield makeValue(i, ascension + 1);
        }
    }
}

const LevelOptions = Array.from(generateLevels()).reverse();

// const AscensionOptions = new Array(7).fill(0).map((_, i) => ({ value: i.toString(), label: `A${i}` })).reverse();
const EidolonOptions = new Array(7).fill(0).map((_, i) => ({ value: i.toString(), label: `E${i}` })).reverse();
const SuperImpositionOptions = new Array(6).fill(0).map((_, i) => ({ value: (i - 1).toString(), label: `S${i}` })).reverse();

function CharacterSelectCard() {
    const [character, setCharacter] = useSession(s => [s.selectedCharacter, s.setSelectedCharacter]);
    const [characterInfo, updateCharacter] = useCharacters(s => [s.getCharacter(character), s.updateCharacter]); //  s.characters[character]) ?? { state: defaultCharacterState, lightCone: undefined };

    return <Card>
        <Column className="gap-4">
            <Column>
                <CardTitle>Character</CardTitle>
                <Row className="gap-2">
                    <Combobox className="w-full min-w-0"
                        modalWidth={300}
                        value={character}
                        onChange={(c) => {
                            setCharacter(c);
                            // setKit(CharacterKitMap[c].defaultConfig)
                        }}
                        deselectable={false}
                        options={[
                            { value: Characters.Jingliu, label: "Jingliu" },
                            // { value: Characters.Xueyi, label: "Xueyi" },
                            { value: Characters.Sparkle, label: "Sparkle" },
                        ]}
                    />
                </Row>

                <Row className="gap-2">
                    <Combobox className="w-full"
                        value={`${characterInfo.state.level};${characterInfo.state.ascension}`}
                        onChange={(c) => {
                            const [level, ascension] = c.split(";");
                            // characterState.level = +level;
                            // characterState.ascension = +ascension;
                            updateCharacter(character, (character) => {
                                character.state.level = +level;
                                character.state.ascension = +ascension;
                            })
                        }}
                        deselectable={false}
                        options={LevelOptions}
                    />

                    <Combobox className="w-[110px]"
                        value={characterInfo.state.eidolon.toString()}
                        onChange={async (c) => {
                            const upgrades = await commands.getEidolonUpgrades(character);
                            const newEidolon = +c;

                            updateCharacter(character, (character) => {
                                const oldEidolon = character.state.eidolon;
                                character.state.eidolon = newEidolon;

                                // Need to apply (or remove) the upgrades
                                if (newEidolon > oldEidolon) {
                                    for (let i = oldEidolon + 1; i <= newEidolon; i++) {
                                        const upgrade = upgrades[i - 1];
                                        character.state.skills.basic  += upgrade.basic;
                                        character.state.skills.skill  += upgrade.skill;
                                        character.state.skills.ult    += upgrade.ult;
                                        character.state.skills.talent += upgrade.talent;
                                    }
                                } else if (newEidolon < oldEidolon) {
                                    for (let i = oldEidolon; i > newEidolon; i--) {
                                        if (i === 0) {
                                            break;
                                        }

                                        const upgrade = upgrades[i - 1];
                                        character.state.skills.basic  -= upgrade.basic;
                                        character.state.skills.skill  -= upgrade.skill;
                                        character.state.skills.ult    -= upgrade.ult;
                                        character.state.skills.talent -= upgrade.talent;
                                    }
                                }
                            })
                        }}
                        deselectable={false}
                        options={EidolonOptions}
                    />
                </Row>
            </Column>

            <Column>
                <CardTitle>Light Cone</CardTitle>
                <Row className="gap-2">
                    <Combobox className="w-full min-w-0"
                        placeholder="No Light Cone Equipped"
                        modalWidth={300}
                        value={characterInfo.lightCone?.[0]}
                        onChange={(c) => {
                            // props.setLightCone(c);
                            updateCharacter(character, (character) => {
                                if (c === undefined) {
                                    character.lightCone = undefined;
                                    return;
                                }

                                if (character.lightCone === undefined) {
                                    character.lightCone = [c, { level: 80, ascension: 6, superimposition: 0 }];
                                } else {
                                    character.lightCone[0] = c;
                                }
                            })
                            // setKit(CharacterKitMap[c].defaultConfig)
                        }}
                        deselectable={true}
                        options={[
                            { value: LightCones.EarthlyEscapade, label: "Earthly Escapade" },
                            { value: LightCones.IShallBeMyOwnSword, label: "I Shall Be My Own Sword" },
                        ]}
                    />
                </Row>

                <Row className="gap-2">
                    <Combobox className="w-full"
                        value={`${characterInfo.lightCone?.[1].level ?? 80};${characterInfo.lightCone?.[1].ascension ?? 6}`}
                        disabled={characterInfo.lightCone === undefined}
                        onChange={(c) => {
                            const [level, ascension] = c.split(";");
                            updateCharacter(character, (character) => {
                                character.lightCone![1].level = +level;
                                character.lightCone![1].ascension = +ascension;
                            })
                        }}
                        deselectable={false}
                        options={LevelOptions}
                    />

                    <Combobox className="w-[110px]"
                        value={characterInfo.lightCone?.[1].superimposition.toString() ?? "0"}
                        disabled={characterInfo.lightCone === undefined}
                        onChange={(c) => {
                            updateCharacter(character, (character) => {
                                character.lightCone![1].superimposition = +c;
                            })
                        }}
                        deselectable={false}
                        options={SuperImpositionOptions}
                    />
                </Row>
            </Column>
        </Column>
    </Card>;
}

type FilterKey = keyof UnionToIntersection<StatFilter>;
type FilterValue = [StatFilterType, number | null, number | null] | [StatColumnType, number | null, number | null];
function FilterRow<K extends FilterKey>(
    props: { 
        key: K,
        label: string, 
        big?: boolean 
    } & (K extends "Action" ? { actionKey: StatColumnType } : {})
) {
    const selectedCharacter = useSession(s => s.selectedCharacter);
    const allFilterData = useCharacters(s => s.getFilterForm(selectedCharacter).statFilters);
    const filterData = findAndMap(allFilterData, f => {
        if (!(props.key in f)) {
            return false;
        }

        if (props.key === "Action") {
            return (f as Extract<StatFilter, {Action: unknown}>).Action[0] === (props as {actionKey: StatColumnType}).actionKey;
        }

        return true;
    }, f => (f as Record<FilterKey, FilterValue>)[props.key as FilterKey]) ?? [null, null, null];

    const { register } = useForm({
        min: filterData[1] ?? "",
        max: filterData[2] ?? "",
    }, v => {
        console.log(v);
    })

    // if (props.big) {
    //     return <Column className="border p-1 rounded-md text-center gap-0">
    //         {props.label}
    //         <Row className="justify-between items-center">
    //             <Row className="gap-2 items-center"><Input className="w-16 p-2 h-6" />&le;</Row>
    //             <span>x</span>
    //             <Row className="gap-2 items-center">&le;<Input className="w-16 p-2 h-6" /></Row>
    //         </Row>
    //     </Column>
    // } else {
    const extraClass = props.big && "text-xs"

        return <Row className="justify-between items-center gap-0">
            <Row className="gap-2 items-center"><Input className={cn("w-16 p-2 h-6", extraClass)} {...register("min")} />&le;</Row>
            <span className={cn(extraClass)}>{props.label}</span>
            <Row className="gap-2 items-center">&le;<Input className={cn("w-16 p-2 h-6", extraClass)} {...register("max")} /></Row>
        </Row>;
    // }
}

function Index() {
    const character = useSession(s => s.selectedCharacter);
    const characterInfo = useCharacters(s => s.getCharacter(character));
    const [filterForm, updateFilterForm] = useCharacters(s => [s.getFilterForm(character), s.updateFilterForm]);
    const [lightCone, lcState] = useCharacters(s => s.characters[character]?.lightCone) ?? [undefined, undefined];

    const [kit, setKit] = useState<CharacterConfig>();
    const [lcKit, setLcKit] = useState<LightConeConfig>();

    const [filters, setFilters] = useState<((r: Relic) => boolean)[]>([])

    const allRelics = useRelics(d => d.relics);
    const filteredRelics = useMemo(() => {
        return allRelics.filter(r => filters.every(f => f(r)));
    }, [allRelics, filters]);

    const [result, setResult] = useCalcs(c => [c.sortResults, c.setSortResults]);
    useEffect(() => setResult(undefined), [character]); // Clear search results when character changes

    const [running, setRunning] = useCalcs(c => [c.running, c.setRunning]);
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
                kit,
                characterInfo.state,
                lcKit && lcState ? [lcKit, lcState] : null,
                {
                    count: 1,
                    level: 95,

                    resistance: 0.2,
                    elemental_weakness: true,
                    weakness_broken: false,
                    debuff_count: 3,
                },
                []
            ));
            setRunning(false);
        }
    };

    const { data: characterActions } = useQuery({
        queryKey: ["characterAction", kit],
        queryFn: () => kit ? commands.getCharacterActions(kit) : undefined,
    })

    return (
        <Column className="min-w-0">
            <h3>Welcome Home!</h3>

            <Row className="flex-wrap gap-2 justify-center">
                <CharacterPreviewCard character={character} lightCone={lightCone} />

                <CharacterSelectCard />

                <CharacterKitCard key={character} character={character} onChange={setKit} />

                <LightConeKitCard key={lightCone} character={character} lightCone={lightCone} onChange={setLcKit} />

                <MainStatFilterCard onChange={fs => setFilters(fs)}/>

                <Card>
                    <CardTitle>Stat Filters</CardTitle>
                    <Column>
                        <ButtonGroup value={filterForm.statType} onChange={v => updateFilterForm(character, f => { f.statType = v })}
                            options={[{ label: "Base Stats", value: "base" }, { label: "Combat Stats", value: "combat" }]}
                        />
                        <Column className="gap-1">
                            <FilterRow label="HP" key="HP" />
                            <FilterRow label="ATK" key="ATK" />
                            <FilterRow label="DEF" key="DEF" />
                            <FilterRow label="SPD" key="SPD" />
                            <FilterRow label="CR" key="CritRate" />
                            <FilterRow label="CD" key="CritDmg" />
                            <FilterRow label="EHR" key="EffectHitRate" />
                            <FilterRow label="ER" key="EffectHitRate" />
                            <FilterRow label="BE" key="BreakEffect" />
                            {/* <FilterRow label="Energy Regen" />
                            <FilterRow label="Heal Boost" /> */}
                            <FilterRow label="ELEM" key="ElementalDmgBoost" />
                        </Column>
                    </Column>
                </Card>
                <Card>
                    <CardTitle>Calculation Filters</CardTitle>
                    <Column className="gap-1">
                        <FilterRow label="CV" key="CritValue" />
                        <FilterRow label="EHP" key="EffectiveHP" />
                        <FilterRow label="Weight" key="Weight" />

                        <Separator className="my-2"/>

                        { characterActions?.map(action =>
                            <FilterRow big label={action[1]} key="Action" actionKey={action[0]} />
                        ) }
                    </Column>
                </Card>

                <PermutationCard
                    allRelics={allRelics}
                    filteredRelics={filteredRelics}
                    triggerSearch={triggerSearch}
                />
            </Row>

            <div className="w-full relative">
                <ScrollArea
                    className="w-full"
                    scrollbar={<ScrollBar orientation="horizontal" />}
                >
                    <OptimizerTable key={character} className="w-full"
                        statType={filterForm.statType === "base" ? 0 : 1}
                        data={result}
                    />
                </ScrollArea>
            </div>
        </Column>
    );
}
