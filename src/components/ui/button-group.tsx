import { Row } from "../util/flex";
import { Button } from "./button";

interface ButtonGroupProps<V extends string> {
    value: V;
    onChange: (value: V) => void;

    options: { label: string, value: V }[];
}

export function ButtonGroup<V extends string>(props: ButtonGroupProps<V>) {
    return <Row className="gap-0">
        {props.options.map((option) => (
            <Button
                size="sm"
                key={option.value}
                variant={props.value === option.value ? "default" : "secondary"}
                className={"flex-1 rounded-none first:rounded-l-md last:rounded-r-md"}
                onClick={() => props.onChange(option.value)}
            >
                {option.label}
            </Button>
        ))}
    </Row>
}
