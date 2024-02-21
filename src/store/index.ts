import { create } from "zustand"
import { persist } from "zustand/middleware"
// import { withLenses, lens, mergeDeep } from "@dhmk/zustand-lens";
import { CharacterState, LightConeState, Relic, SortResultsSerde, StatFilter } from "@/bindings.gen";
import { Characters } from "@/kits/characters";
import { LightCones } from "@/kits/light-cones";
import { Draft, produce } from "immer";

type Theme = "light" | "dark" | "system"
export const useSettings = create<{
    theme: Theme
    setTheme: (theme: Theme) => void
}>()(persist((set) => ({
    theme: "system",
    setTheme: (theme) => set({ theme }),
}), { name: "settings" }))

export const useRelics = create<{
    relics: Relic[]
    setRelics: (relics: Relic[]) => void
}>()(persist((set) => ({
    relics: [] as Relic[],
    setRelics: (relics) => set({ relics }),
}), { name: "relics" }))

interface FilterForm {
    statType: "base" | "combat"
    statFilters: StatFilter[]
}

interface MyCoolCharacterInformation {
    state: CharacterState,
    lightCone: [LightCones, LightConeState] | undefined,
    filterForm?: FilterForm,
}

const defaultCharacterState = {
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

const defaultCharacterInfo: MyCoolCharacterInformation = { state: defaultCharacterState, lightCone: undefined }
const defaultFilterForm: FilterForm = { statType: "combat", statFilters: [] }

export const useCharacters = create<{
    characters: Partial<Record<Characters, MyCoolCharacterInformation>>,
    setCharacters: (characters: Partial<Record<Characters, MyCoolCharacterInformation>>) => void
    getCharacter: (character: Characters) => MyCoolCharacterInformation
    updateCharacter: (character: Characters, updater: (character: Draft<MyCoolCharacterInformation>) => void) => void
    getFilterForm: (character: Characters) => FilterForm
    updateFilterForm: (character: Characters, updater: (filterForm: Draft<FilterForm>) => void) => void
}>()(persist((set, get) => ({
    characters: {},
    setCharacters: (characters) => set({ characters }),
    getCharacter: (character: Characters) => {
        return get().characters[character] ?? defaultCharacterInfo
    },
    updateCharacter: (character, updater) => {
        set((state) => {
            const characterState = state.characters[character] ?? defaultCharacterInfo
            const newState = produce(characterState, updater)

            return {
                characters: {
                    ...state.characters,
                    [character]: newState,
                }
            }
        })
    },
    getFilterForm: (character) => {
        return get().characters[character]?.filterForm ?? defaultFilterForm
    },
    updateFilterForm: (character, updater) => {
        get().updateCharacter(character, (character) => {
            character.filterForm = produce(character.filterForm ?? defaultFilterForm, updater)
        })
    }
}), { name: "characters" }))

export const useSession = create<{
    selectedCharacter: Characters
    setSelectedCharacter: (selectedCharacter: Characters) => void
}>()(persist((set) => ({
    selectedCharacter: Characters.Jingliu,
    setSelectedCharacter: (selectedCharacter) => set({ selectedCharacter }),
}), { name: "session" }))

export const useCalcs = create<{
    sortResults: SortResultsSerde | undefined
    setSortResults: (sortResults: SortResultsSerde | undefined) => void

    running: boolean
    setRunning: (running: boolean) => void
}>()((set) => ({
    sortResults: undefined,
    setSortResults: (sortResults) => set({ sortResults }),

    running: false,
    setRunning: (running) => set({ running }),
}))

// export const useStore = create<{
//     settings: typeof settingsSlice
//     data: typeof dataSlice
// }>()(persist(
//     withLenses(() => ({
//         settings: settingsSlice,
//         data: dataSlice,
//     })),
//     {
//         name: "store",
//         merge: (a: any, b) => mergeDeep(b, a),
//     }
// ))
