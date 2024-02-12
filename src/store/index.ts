import { create } from "zustand"
import { persist } from "zustand/middleware"
// import { withLenses, lens, mergeDeep } from "@dhmk/zustand-lens";
import { CharacterState, LightConeState, Relic, SortResultsSerde } from "@/bindings.gen";
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
    relics: [],
    setRelics: (relics) => set({ relics }),
}), { name: "relics" }))

interface MyCoolCharacterInformation {
    state: CharacterState,
    lightCone: [LightCones, LightConeState] | undefined,
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

export const useCharacters = create<{
    characters: Partial<Record<Characters, MyCoolCharacterInformation>>,
    setCharacters: (characters: Partial<Record<Characters, MyCoolCharacterInformation>>) => void
    getCharacter: (character: Characters) => MyCoolCharacterInformation
    updateCharacter: (character: Characters, updater: (character: Draft<MyCoolCharacterInformation>) => void) => void
}>()(persist((set, get) => ({
    characters: {},
    setCharacters: (characters) => set({ characters }),
    getCharacter: (character: Characters) => {
        return get().characters[character] ?? { state: defaultCharacterState, lightCone: undefined }
    },
    updateCharacter: (character, updater) => {
        set((state) => {
            const characterState = state.characters[character] ?? { state: defaultCharacterState, lightCone: undefined }
            const newState = produce(characterState, updater)

            return {
                characters: {
                    ...state.characters,
                    [character]: newState,
                }
            }
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
    setSortResults: (sortResults: SortResultsSerde) => void

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
