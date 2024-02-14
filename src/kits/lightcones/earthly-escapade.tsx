import { EarthlyEscapadeConfig, LightConeState } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { useForm } from "@/utils/form";

export const EarthlyEscapadeDefaultConfig: EarthlyEscapadeConfig = {
    has_mask: true,
}

export function EarthlyEscapadeKit(props: {
    lightConeState: LightConeState,
    value: EarthlyEscapadeConfig,
    onChange: (value: EarthlyEscapadeConfig) => void
}) {
    const {
        registerSwitch,
    } = useForm<EarthlyEscapadeConfig>(props.value, props.onChange);

    return <Column>
        <LabeledSwitch {...registerSwitch("has_mask")} label="Has Mask" />
    </Column>
}
