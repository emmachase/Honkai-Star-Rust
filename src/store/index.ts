import { create } from "zustand"
import { persist } from "zustand/middleware"
// import { withLenses, lens, mergeDeep } from "@dhmk/zustand-lens";
import { Relic, SortResultsSerde } from "@/bindings.gen";

type Theme = "light" | "dark" | "system"
export const useSettings = create<{
    theme: Theme
    setTheme: (theme: Theme) => void
}>()(persist((set) => ({
    theme: "system",
    setTheme: (theme) => set({ theme }),
}), { name: "settings" }))

export const useData = create<{
    relics: Relic[]
    setRelics: (relics: Relic[]) => void
}>()(persist((set) => ({
    relics: [],
    setRelics: (relics) => set({ relics }),
}), { name: "data" }))

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
