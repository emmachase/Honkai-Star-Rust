import { cn } from "@/utils";
import React from "react";

export const Row = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
    ({ className, ...props }, ref) => (
        <div className={cn("flex flex-row gap-4", className)} {...props} ref={ref} />
    )
)
Row.displayName = "Row"

export const Column = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
    ({ className, ...props }, ref) => (
        <div className={cn("flex flex-col gap-2", className)} {...props} ref={ref} />
    )
)
Column.displayName = "Column"
