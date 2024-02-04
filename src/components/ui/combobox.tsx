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
import { useRef } from "react";

export function Combobox<V extends string>(
    props: {
        label?: string;
        options: {
            value: V;
            label: string;
        }[];
        placeholder?: string;
        className?: string;
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

    let isPlaceholder = true;
    let content = props.placeholder ?? "Select an option...";
    if (multiple) {
        if (props.value.length > 0) {
            isPlaceholder = false;
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

            isPlaceholder = false;
            content = option?.label ?? "Your code is broken";
        }
    }

    const triggerRef = useRef<HTMLButtonElement>(null);
    const [triggerWidth, setTriggerWidth] = React.useState(250);

    return (<>
        <div className="text-sm">{props.label}</div>
        <Popover open={open} onOpenChange={(o) => {
            if (o) {
                setTriggerWidth(triggerRef.current?.offsetWidth ?? 250);
            }
            setOpen(o);
        }}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className={cn("w-[250px] justify-between", props.className)}
                    ref={triggerRef}
                >
                    <span className={cn("overflow-ellipsis overflow-hidden", isPlaceholder && "text-muted-foreground")}>
                        {content}
                    </span>
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="p-0" style={{ width: triggerWidth }}>
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
    </>);
}
