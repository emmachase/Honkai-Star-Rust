import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";
import { produce } from "immer";

import { Button } from "@/components/ui/button";
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "@/components/ui/command";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/utils";

export function Combobox<V extends string>(
    props: {
        options: {
            value: V;
            label: string;
        }[];
        placeholder?: string;
    } & (
        | {
              deselectable?: true;
              value: V | undefined;
              onChange: (value: V | undefined) => void;
          }
        | {
              deselectable: false;
              value: V;
              onChange: (value: V) => void;
          }
        | {
              multiple: true;
              value: V[];
              onChange: (value: V[]) => void;
          }
    ),
) {
    const [open, setOpen] = React.useState(false);
    const multiple = "multiple" in props;

    let content = props.placeholder ?? "Select an option...";
    if (multiple) {
        if (props.value.length > 0) {
            content = props.value
                .map((value) =>
                    props.options.find((option) => option.value === value)?.label,
                )
                .join(", ");
        }
    } else {
        if (props.value) {
            const option = props.options.find(
                (option) => option.value === props.value,
            );

            content = option?.label ?? "Your code is broken";
        }
    }

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[250px] justify-between"
                >
                    <span className="overflow-ellipsis overflow-hidden">
                        {content}
                    </span>
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[250px] p-0">
                <Command>
                    <CommandInput placeholder="Search" />
                    <CommandEmpty>No results found.</CommandEmpty>
                    <CommandList>
                        {props.options.map((option) => (
                            <CommandItem
                                key={option.value}
                                value={option.value}
                                keywords={[option.label]}
                                onSelect={(currentValue) => {
                                    if (multiple) {
                                        props.onChange(
                                            produce(props.value as string[], (draft) => {
                                                if (draft.includes(option.value)) {
                                                    draft.splice(
                                                        draft.indexOf(option.value),
                                                        1,
                                                    );
                                                } else {
                                                    draft.push(option.value);
                                                }
                                            }) as V[],
                                        );
                                    } else {
                                        if ("deselectable" in props && !props.deselectable) {
                                            props.onChange(currentValue as V);
                                        } else {
                                            props.onChange(
                                                (currentValue === props.value
                                                    ? undefined
                                                    : currentValue) as V,
                                            );
                                        }
                                    }

                                    if (!multiple) {
                                        setOpen(false);
                                    }
                                }}
                            >
                                <Check
                                    className={cn(
                                        "mr-2 h-4 w-4",
                                        (multiple ? props.value.includes(option.value) : props.value === option.value)
                                            ? "opacity-100"
                                            : "opacity-0",
                                    )}
                                />
                                {option.label}
                            </CommandItem>
                        ))}
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    );
}
