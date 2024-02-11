import { CharacterState, JingliuConfig } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { SliderWithInput } from "@/components/ui/slider";
import { useDescriptions } from "../utils";
import { useForm } from "@/utils/form";

export const JingliuDefaultConfig: JingliuConfig = {
    enhanced_state: true,
    e1_crit_dmg: true,
    e2_skill_buff: true,
    hp_drain_pct: 1.0,
}

export function JingliuKit(props: {
    characterState: CharacterState,
    value: JingliuConfig,
    onChange: (value: JingliuConfig) => void
}) {
    const descriptions = useDescriptions("Jingliu");

    const {
        register,
        registerSwitch,
    } = useForm<JingliuConfig>(props.value, props.onChange);

    const e = props.characterState.eidolon;
    let hpDrainCap = descriptions.talent[props.characterState.skills.talent].atk_pct_cap;
    if (e >= 4) hpDrainCap += 0.3;

    const enhanced = props.value.enhanced_state;

    return <Column>
        <LabeledSwitch {...registerSwitch("enhanced_state")}
            label="Enhanced State"
        />

        <SliderWithInput {...register("hp_drain_pct")} percent
            label="HP Drain ATK Buff"
            disabled={!enhanced}
            displayMax={hpDrainCap}
        />

        <LabeledSwitch {...registerSwitch("e1_crit_dmg")}
            label="E1 Post-Ult Crit DMG"
            disabled={e < 1}
        />

        <LabeledSwitch {...registerSwitch("e2_skill_buff")}
            label="E2 Skill DMG Boost"
            disabled={e < 2 || !enhanced}
        />
    </Column>
}
