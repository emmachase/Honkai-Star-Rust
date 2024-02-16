import { CharacterState, SparkleBaseConfig, SparkleTeammateConfig } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { SliderWithInput } from "@/components/ui/slider";
import { Form, useForm } from "@/utils/form";
import { LabeledNumberInput } from "@/components/ui/input";

SparkleKit.defaultConfig = {
    skill_cd_buff: true,
    cipher_buff: true,
    talent_dmg_stacks: 3,
    quantum_allies: 3,
} as SparkleBaseConfig;

SparkleTeammateKit.defaultConfig = {
    cd_stat: 2.4,
    skill_cd_buff: true,
    cipher_buff: true,
    talent_dmg_stacks: 3,
    quantum_allies: 3,
} as SparkleTeammateConfig;

function sharedOptions({
    register,
    registerSwitch,
}: Form<SparkleBaseConfig & SparkleTeammateConfig>) {
    return <>
        <LabeledSwitch {...registerSwitch("skill_cd_buff")}
            label="Skill CD Buff"
        />

        <LabeledSwitch {...registerSwitch("cipher_buff")}
            label="Cipher Buff"
        />

        <SliderWithInput {...register("talent_dmg_stacks")}
            label="Talent DMG Stacks"
            max={3}
        />

        <SliderWithInput {...register("quantum_allies")}
            label="Quantum Allies"
            max={3}
        />
    </>
}

export function SparkleKit(props: {
    characterState: CharacterState,
    value: SparkleBaseConfig,
    onChange: (value: SparkleBaseConfig) => void
}) {
    const form = useForm<SparkleBaseConfig>(props.value, props.onChange);

    return <Column>
        {sharedOptions(form)}
    </Column>
}

export function SparkleTeammateKit(props: {
    characterState: CharacterState,
    value: SparkleTeammateConfig,
    onChange: (value: SparkleTeammateConfig) => void
}) {
    const form = useForm<SparkleTeammateConfig>(props.value, props.onChange), {
        register,
    } = form;

    return <Column>
        <LabeledNumberInput {...register("cd_stat")} percent min={0.5}
            label="Sparkle's CD"
        />

        {sharedOptions(form)}
    </Column>
}
