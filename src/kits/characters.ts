import { CharacterConfig, CharacterState } from "@/bindings.gen";
import { JingliuKit } from "./characters/jingliu";
import { SparkleKit, SparkleTeammateKit } from "./characters/sparkle";

export enum Characters {
    Jingliu = "Jingliu",
    // Xueyi = "Xueyi",
    Sparkle = "Sparkle",
}

export interface CharacterKitComponent<Config> {
    (props: {
        characterState: CharacterState,
        value: Config,
        onChange: (value: Config) => void
    }): JSX.Element
}

interface CharacterKitShit<Config> {
    component: CharacterKitComponent<Config>
    defaultConfig: Config
    wrapConfig: (config: Config) => CharacterConfig
}

export const CharacterKitMap = {
    [Characters.Jingliu]: {
        component: JingliuKit,
        defaultConfig: JingliuKit.defaultConfig,
        wrapConfig: c => ({ Jingliu: c }),
    },
    [Characters.Sparkle]: {
        component: SparkleKit,
        defaultConfig: SparkleKit.defaultConfig,
        wrapConfig: c => ({ Sparkle: { Own: c } }),
    },
} satisfies Record<Characters, CharacterKitShit<any>>
