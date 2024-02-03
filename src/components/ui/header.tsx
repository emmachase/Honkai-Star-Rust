import { cn } from "@/utils";
import { PropsWithChildren } from "react";

export type HeaderLevel = 1 | 2 | 3 | 4 | 5 | 6;
const levelClass = {
    1: "text-4xl",
    2: "text-3xl",
    3: "text-2xl",
    4: "text-xl",
    5: "text-lg",
    6: "text-base",
};

export function Header(props: PropsWithChildren<{ level: HeaderLevel }> & React.HTMLAttributes<HTMLHeadingElement>) {
    const Tag = `h${props.level}` as const;

    return <Tag {...props} className={cn(levelClass[props.level], props.className)}>{props.children}</Tag>;
}
