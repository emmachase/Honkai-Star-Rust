import { EarthlyEscapadeConfig, LightConeState } from "@/bindings.gen";
import { LabeledSwitch } from "@/components/ui/switch";
import { Column } from "@/components/util/flex";
import { useForm } from "@/utils/form";

EarthlyEscapadeKit.defaultConfig = {
    has_mask: true,
} as EarthlyEscapadeConfig;

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
