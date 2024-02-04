import * as React from "react"

import { cn } from "@/utils"
import { Row } from "../util/flex"
import { useEffect, useState } from "react";

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const BufferedInput = React.forwardRef<HTMLInputElement, InputProps>(
    ({ onChange, value, ...props }, ref) => {
        const [bufferedValue, setBufferedValue] = useState(value);
        useEffect(() => {
            setBufferedValue(value);
        }, [value]);

        return (
            <input
                ref={ref}
                value={bufferedValue}
                onChange={(e) => {
                    setBufferedValue(e.currentTarget.value);
                }}
                onBlur={(e) => {
                    onChange?.(e);
                }}
                {...props}
            />
        );
    }
)

const Input = React.forwardRef<HTMLInputElement, InputProps & { raw?: boolean }>(
  ({ className, type, raw, ...props }, ref) => {
    const Component = raw ? "input" : BufferedInput;

    return (
      <Component
        type={type}
        className={cn(
          "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Input.displayName = "Input"

const SuffixInput = React.forwardRef<HTMLInputElement, InputProps & { suffix: string }>(
    ({ className, suffix, type, ...props }, ref) => {
        return (<Row className={cn("h-10 gap-0 overflow-hidden rounded-md border border-input ring-offset-background focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50", className)} ref={ref}>
            <BufferedInput
                type={type}
                className={cn(
                "flex h-full w-full bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none",
                )}
                ref={ref}
                {...props}
            />
            <div className="h-full px-2 bg-border font-bold grid place-items-center">
                {suffix}
            </div>
        </Row>)
    }
)

export { Input, SuffixInput }
