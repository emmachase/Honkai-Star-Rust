import { IShallBeMyOwnSwordConfig, LightConeState } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { SliderWithInput } from "@/components/ui/slider";
import { useForm } from "@/utils/form";

export const IShallBeMyOwnSwordDefaultConfig: IShallBeMyOwnSwordConfig = {
    eclipse_stacks: 3,
    max_stack_def_pen: true,
}

export function IShallBeMyOwnSwordKit(props: {
    lightConeState: LightConeState,
    value: IShallBeMyOwnSwordConfig,
    onChange: (value: IShallBeMyOwnSwordConfig) => void
}) {
    const {
        register,
        registerSwitch,
    } = useForm<IShallBeMyOwnSwordConfig>(props.value, props.onChange);

    return <Column>
        <SliderWithInput {...register("eclipse_stacks")} max={3} label="Eclipse Stacks" />
        <LabeledSwitch {...registerSwitch("max_stack_def_pen")} label="Max Stack DEF Pen" />
    </Column>
}
