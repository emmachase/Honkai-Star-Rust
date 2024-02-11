import * as React from "react"
import * as SliderPrimitive from "@radix-ui/react-slider"

import { cn } from "@/utils"
import { Column, Row } from "../util/flex"
import { Input, SuffixInput } from "./input"
import { NaNTo, clamp } from "@/utils/math"
import { useCallback } from "react"

const Slider = React.forwardRef<
  React.ElementRef<typeof SliderPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof SliderPrimitive.Root>
>(({ className, ...props }, ref) => (
  <SliderPrimitive.Root
    ref={ref}
    className={cn(
      "relative flex w-full touch-none select-none items-center data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
      className
    )}
    {...props}
  >
    <SliderPrimitive.Track className="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
      <SliderPrimitive.Range className="absolute h-full bg-primary" />
    </SliderPrimitive.Track>
    <SliderPrimitive.Thumb className="block h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2" />
  </SliderPrimitive.Root>
))
Slider.displayName = SliderPrimitive.Root.displayName

export function SliderWithInput({label, disabled, value, onChange, min, max, displayMin, displayMax, percent, step, ...props}: Omit<React.ComponentPropsWithoutRef<typeof Slider>, "value" | "onValueChange" | "onChange"> & { displayMin?: number, displayMax?: number, percent?: boolean, value?: number, onChange?: (value: number) => void, label: string }) {
    min ??= 0
    max ??= 1
    const percentScaling = percent ? 100 : 1
    displayMin ??= min
    displayMax ??= max
    displayMin *= percentScaling
    displayMax *= percentScaling
    step ??= 1

    const roundToStep = useCallback((x: number) => Math.round(x / step!) * step!, [step])

    value ??= min

    const domain = max - min
    const range = displayMax - displayMin
    const rescaledValue = displayMin + roundToStep(range*(value - min)/domain)

    const inputProps: Partial<React.ComponentPropsWithRef<typeof Input>> = {
        className: "h-8",
        disabled: disabled,
        value: rescaledValue,
        onChange: x => {
            onChange?.(clamp(min! + domain*roundToStep(NaNTo(+x.currentTarget.value, displayMin!) - displayMin!)/range, min!, max!))
        }
    }

    return (<Column>
        <Row className="items-center">
            { percent
                ? <SuffixInput suffix="%" {...inputProps} />
                : <Input {...inputProps} />
            }
            <span className="whitespace-nowrap">{label}</span>
        </Row>

        <Row className="items-center">
            <Slider
                disabled={disabled}
                min={displayMin}
                max={displayMax}
                value={[rescaledValue]}
                onValueChange={([x]) => {
                    onChange?.(min! + domain*roundToStep(x - displayMin!)/range)
                    console.log(`${min} + ${domain} * (${x} - ${displayMin}) / ${range} = ${min! + domain*(x - displayMin!)/range}`)
                }}

                {...props}
            />
            <span>{displayMax}{percent && "%"}</span>
        </Row>
    </Column>)
}

export { Slider }
