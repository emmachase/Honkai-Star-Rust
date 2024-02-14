import { CharacterState, SparkleBaseConfig } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { SliderWithInput } from "@/components/ui/slider";
import { useForm } from "@/utils/form";

export const SparkleDefaultConfig: SparkleBaseConfig = {
    skill_cd_buff: true,
    cipher_buff: true,
    talent_dmg_stacks: 3,
    quantum_allies: 3,
}

export function SparkleKit(props: {
    characterState: CharacterState,
    value: SparkleBaseConfig,
    onChange: (value: SparkleBaseConfig) => void
}) {
    const {
        register,
        registerSwitch,
    } = useForm<SparkleBaseConfig>(props.value, props.onChange);

    return <Column>
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
    </Column>
}
