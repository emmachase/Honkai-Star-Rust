import { CharacterState, JingliuConfig } from "@/bindings.gen";
import { LabeledSwitch, Switch } from "@/components/ui/switch";
import { useState, SyntheticEvent, ChangeEvent } from "react";
import { produce } from "immer";
import { Column, Row } from "@/components/util/flex";
import { Slider } from "@/components/ui/slider";
import { SuffixInput } from "@/components/ui/input";
import { NaNTo, clamp } from "@/utils/math";
// import { useForm } from "react-hook-form"

interface RegisterConfig<F, To = F> {
    valueName?: string
    onChangeName?: string

    toForm?: (value: To) => F
    fromForm?: (value: F) => To
}

function useForm<T extends Record<string, unknown>>(value: T, setValue: (value: T) => void) {
    const myForm = {
        register<K extends keyof T, F>(key: K, config?: RegisterConfig<F, T[K]>) {
            return {
                [config?.valueName ?? "value"]: (config?.toForm ?? (x => x))(value[key]),
                [config?.onChangeName ?? "onChange"]: (fieldValue: T[keyof T] | SyntheticEvent<HTMLInputElement, ChangeEvent<HTMLInputElement>>) => {
                    setValue(produce(value, (draft) => {
                        if (fieldValue && typeof fieldValue === "object" && "nativeEvent" in fieldValue) {
                            fieldValue = fieldValue.currentTarget.value as T[keyof T];
                        }

                        (draft as T)[key] = (config?.fromForm ?? (x => x))(fieldValue as F) as T[K];
                    }));
                }
            }
        },

        registerSlider<K extends keyof T, F>(key: K, config?: RegisterConfig<F, T[K]>) {
            return myForm.register(key, {
                ...config,
                onChangeName: "onValueChange",
                toForm: (x: T[K]) => [(config?.toForm ?? (x => x))(x)] as F[],
                fromForm: (x: F[]) => (config?.fromForm ?? (x => x))(x[0]) as T[K],
            });
        },

        registerSwitch<K extends keyof T, F>(key: K, config?: RegisterConfig<F, T[K]>) {
            return myForm.register(key, {
                ...config,
                valueName: "checked",
                onChangeName: "onCheckedChange"
            });
        }
    }

    return myForm;
}

export function JingliuKit(props: {
    characterState: CharacterState,
    value: JingliuConfig,
    onChange: (value: JingliuConfig) => void
}) {
    const {
        register,
        registerSwitch,
        registerSlider
        // watch,
        // setValue
    } = useForm<JingliuConfig>(props.value, props.onChange);
    //     {
    //     defaultValues: {
    //         enhanced_state: true,
    //         e1_crit_dmg: true,
    //         e2_skill_buff: true,
    //         hp_drain_pct: 1.0
    //     },
    //     values: props.value
    // });

    // watch(state => {
    //     console.log(state)
    //     props.onChange(state as JingliuConfig);
    // })

    const e = props.characterState.eidolon;
    let hpDrainCap = e >= 4 ? 228 : e >= 3 ? 198 : 180;

    return <Column>
        <LabeledSwitch {...registerSwitch("enhanced_state")}
            label="Enhanced State"
        />

        {/* <input type="number" {...register("hp_drain_pct", {
            toForm: x => x * 100,
            fromForm: x => x / 100
        })} /> */}
        <Row className="items-center">
            <SuffixInput suffix="%" className="h-8"
                {...register("hp_drain_pct", {
                    toForm: x => Math.round(x * hpDrainCap),
                    fromForm: x => clamp(NaNTo(x / hpDrainCap, 1), 0, 1)
                })}
            />
            <span className="whitespace-nowrap">HP drain ATK buff</span>
        </Row>

        <Row className="items-center">
            <Slider
                {...registerSlider("hp_drain_pct")}
                min={0}
                max={1}
                step={1 / hpDrainCap}
            />
            <span>{hpDrainCap}%</span>
        </Row>

        <LabeledSwitch {...registerSwitch("e1_crit_dmg")}
            label="E1 Crit DMG"
            disabled={e < 1}
        />

        <LabeledSwitch {...registerSwitch("e2_skill_buff")}
            label="E2 Skill Buff"
            disabled={e < 2}
        />
    </Column>
}
