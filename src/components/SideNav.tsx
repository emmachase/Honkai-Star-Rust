import { Flex, Item } from "@adobe/react-spectrum";
import React from "react";
import { PropsWithChildren } from "react";
import { Button } from "react-aria-components";
import { isReactElement } from "../util/node";

interface SideNavContextValue {
    selected: string;
    setSelected: (selected: string) => void;
}

const SideNavContext = React.createContext<SideNavContextValue>({} as SideNavContextValue);

export function SideNavButton(props: PropsWithChildren<{ id: string }>) {
    const { selected, setSelected } = React.useContext(SideNavContext);
    
    return <Button
        className="w-full text-left rounded-md outline-none ring-inset ring-blue-500 focus-visible:ring-2 hocus:bg-neutral-700 selected:bg-neutral-700"
        data-selected={selected === props.id || null}
        onPress={() => setSelected(props.id)}
    >
        <Flex marginX="size-100" marginY="size-75" alignItems="center" gap="size-100">
            {props.children}
        </Flex>
    </Button>
}

export function SideNav(props: { 
    children: React.ReactNode,
    default: string,
}) {
    const [selected, setSelected] = React.useState(props.default);

    return <SideNavContext.Provider value={{ selected, setSelected }}>
        <Flex direction="column" gap="size-100">
            {props.children}
        </Flex>
    </SideNavContext.Provider>
}
