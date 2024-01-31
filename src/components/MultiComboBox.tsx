import { Badge, Button, ComboBox, Flex, Item, Key } from "@adobe/react-spectrum";
import Close from "@spectrum-icons/workflow/Close";
import CloseCircle from "@spectrum-icons/workflow/CloseCircle";
import { useEffect, useMemo, useState } from "react";

export function MultiComboBox<V extends string = string>(props: {
    label: string,
    options: {label: string, value: V}[],
}) {
    const [selectedKeys, setSelectedKeys] = useState<V[]>([])
    const [selectionHack, setSelectionHack] = useState<V | null>(null)
    if (selectionHack !== null) {
        queueMicrotask(() => {
            setSelectionHack(null)
        })
    }

    const labelMap = useMemo(() => {
        const map = new Map<V, string>()
        for (const o of props.options) {
            map.set(o.value, o.label)
        }
        return map
    }, [props.options]);

    return (
        <Flex alignItems={"end"} gap={"size-200"}>
            <ComboBox label={props.label} selectedKey={selectionHack} onSelectionChange={k => {
                if (selectedKeys.includes(k as V)) {
                    setSelectedKeys((keys) => keys.filter(k2 => k2 !== k))
                } else {
                    setSelectedKeys([...selectedKeys, k as V])
                }
                setSelectionHack(k as V)
            }}>
                {/* <Item key="CRIT Rate">CRIT Rate</Item>
                <Item key="CRIT DMG">CRIT DMG</Item>
                <Item key="ATK%">ATK%</Item> */}
                {props.options.map(o => <Item key={o.value}>{o.label}</Item>)}
            </ComboBox>
            {/* <ul>
                {selectedKeys.map(k => <li key={k}>{k}</li>)}
            </ul> */}
            <Flex gap={"size-100"}>
                {selectedKeys.map(k =>
                    <Button variant="secondary" key={k} onPress={() => {
                        setSelectedKeys((keys) => keys.filter(k2 => k2 !== k))
                    }}>
                        <Flex alignItems={"start"}>
                            <Close size="XXL" marginStart={"size-50"}/>
                            <span className="pl-1 pr-2"> {labelMap.get(k)}</span>
                        </Flex>
                    </Button>
                )}
            </Flex>
        </Flex>
    )
}
