import { ComboBox, Item, Key } from "@adobe/react-spectrum";
import { useEffect, useState } from "react";

export function MultiComboBox(props: {
    label: string
}) {
    const [selectedKeys, setSelectedKeys] = useState<Key[]>([])
    const [selectionHack, setSelectionHack] = useState<Key | null>(null)
    if (selectionHack !== null) {
        queueMicrotask(() => {
            setSelectionHack(null)
        })
    }

    return (
        <>
            <ComboBox label={props.label} selectedKey={selectionHack} onSelectionChange={k => {
                setSelectedKeys([...selectedKeys, k])
                setSelectionHack(k)
            }}>
                <Item key="1">One</Item>
                <Item key="2">Two</Item>
                <Item key="3">Three</Item>
            </ComboBox>
            <ul>
                {selectedKeys.map(k => <li key={k}>{k}</li>)}
            </ul>
        </>
    )
}
