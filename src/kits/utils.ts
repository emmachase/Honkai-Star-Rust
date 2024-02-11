import { Character, CharacterDescriptions, commands } from "@/bindings.gen";
import { useSuspenseQuery } from "@tanstack/react-query";

export function useDescriptions<C extends Character>(character: C): Extract<CharacterDescriptions, Record<C, any>>[C] {
    return (useSuspenseQuery({
        queryKey: ["descriptions", character],
        queryFn: async () => {
            return await commands.getDescription(character);
        }
    }).data as Record<string, unknown>)[character] as Extract<CharacterDescriptions, Record<C, any>>[C];
}
