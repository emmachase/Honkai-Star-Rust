export function findAndMap<T, R>(
  arr: T[],
  predicate: (item: T, index: number, arr: T[]) => boolean,
  mapper: (item: T) => R
): R | undefined {
    const item = arr.find(predicate);
    if (item) {
        return mapper(item);
    }

    return undefined;
}
