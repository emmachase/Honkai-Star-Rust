import { LightConeConfig, LightConeState } from "@/bindings.gen";
import { IShallBeMyOwnSwordKit } from "./lightcones/i-shall-be-my-own-sword";
import { EarthlyEscapadeKit } from "./lightcones/earthly-escapade";

export enum LightCones {
    IShallBeMyOwnSword = "IShallBeMyOwnSword",
    EarthlyEscapade = "EarthlyEscapade",
}

export interface LightConeKitComponent<Config> {
    (props: {
        lightConeState: LightConeState,
        value: Config,
        onChange: (value: Config) => void
    }): JSX.Element
}

interface LightConeKitShit<Config> {
    component: LightConeKitComponent<Config>
    defaultConfig: Config
    wrapConfig: (config: Config) => LightConeConfig
}

export const LightConeKitMap = {
    [LightCones.IShallBeMyOwnSword]: {
        component: IShallBeMyOwnSwordKit,
        defaultConfig: IShallBeMyOwnSwordKit.defaultConfig,
        wrapConfig: c => ({ IShallBeMyOwnSword: c }),
    },
    [LightCones.EarthlyEscapade]: { // TODO: Implement EarthlyEscapade
        component: EarthlyEscapadeKit,
        defaultConfig: EarthlyEscapadeKit.defaultConfig,
        wrapConfig: c => ({ EarthlyEscapade: c }),
    },
} satisfies Record<LightCones, LightConeKitShit<any>>
