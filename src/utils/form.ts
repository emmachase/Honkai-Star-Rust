import { produce } from "immer";
import { SyntheticEvent, ChangeEvent } from "react";

export interface RegisterConfig<
    F,
    ValueName extends string = "value",
    ChangeName extends string = "onChange",
    To = F,
> {
    valueName?: ValueName;
    onChangeName?: ChangeName;

    toForm?: (value: To) => F;
    fromForm?: (value: F) => To;
}

export function useForm<T extends Record<string, unknown>>(
    value: T,
    setValue: (value: T) => void,
) {
    const myForm = {
        registerGeneric<
            K extends keyof T,
            F,
            ValueName extends string = "value",
            ChangeName extends string = "onChange",
        >(
            key: K,
            config?: RegisterConfig<F, ValueName, ChangeName, T[K]>,
        ): { [k in ValueName]: T[K] } & {
            [k in ChangeName]: (fieldValue: T[K]) => void;
        } {
            return {
                [config?.valueName ?? "value"]: (config?.toForm ?? ((x) => x))(
                    value[key],
                ),
                [config?.onChangeName ?? "onChange"]: (
                    fieldValue:
                        | T[keyof T]
                        | SyntheticEvent<
                              HTMLInputElement,
                              ChangeEvent<HTMLInputElement>
                          >,
                ) => {
                    setValue(
                        produce(value, (draft) => {
                            if (
                                fieldValue &&
                                typeof fieldValue === "object" &&
                                "nativeEvent" in fieldValue
                            ) {
                                fieldValue = fieldValue.currentTarget
                                    .value as T[keyof T];
                            }

                            (draft as T)[key] = (
                                config?.fromForm ?? ((x) => x)
                            )(fieldValue as F) as T[K];
                        }),
                    );
                },
            } as { [k in ValueName]: T[K] } & {
                [k in ChangeName]: (fieldValue: T[K]) => void;
            };
        },

        register<K extends keyof T, F>(
            key: K,
            config?: RegisterConfig<F, "value", "onChange", T[K]>,
        ) {
            return myForm.registerGeneric(key, config);
        },

        registerSlider<K extends keyof T, F>(
            key: K,
            config?: RegisterConfig<F, "value", "onValueChange", T[K]>,
        ) {
            return myForm.registerGeneric(key, {
                ...config,
                onChangeName: "onValueChange",
                toForm: (x: T[K]) => [(config?.toForm ?? ((x) => x))(x)] as F[],
                fromForm: (x: F[]) =>
                    (config?.fromForm ?? ((x) => x))(x[0]) as T[K],
            });
        },

        registerSwitch<K extends keyof T, F>(
            key: K,
            config?: RegisterConfig<F, "checked", "onCheckedChange", T[K]>,
        ) {
            return myForm.registerGeneric(key, {
                ...config,
                valueName: "checked",
                onChangeName: "onCheckedChange",
            });
        },
    };

    return myForm;
}

export type Form<T extends Record<string, unknown>> = ReturnType<typeof useForm<T>>
