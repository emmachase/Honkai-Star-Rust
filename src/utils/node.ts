export function isReactElement(element: unknown): element is React.ReactElement {
    return element !== null && typeof element === 'object' && 'type' in element;
}
