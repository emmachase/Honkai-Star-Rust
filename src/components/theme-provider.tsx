import { useSettings } from "@/store";
import { createContext, useContext, useEffect, useState } from "react";

type Theme = "dark" | "light" | "system";

type ThemeProviderProps = {
    children: React.ReactNode;
    defaultTheme?: Theme;
    storageKey?: string;
};

type ThemeProviderState = {
    theme: Theme;
    evaluatedTheme?: Exclude<Theme, "system">;
    setTheme: (theme: Theme) => void;
};

const initialState: ThemeProviderState = {
    theme: "system",
    setTheme: () => null,
};

const ThemeProviderContext = createContext<ThemeProviderState>(initialState);

function useMediaQuery(query: string) {
    const [matches, setMatches] = useState(false);

    useEffect(() => {
        const mediaQuery = window.matchMedia(query);
        setMatches(mediaQuery.matches);

        const listener = () => setMatches(mediaQuery.matches);
        mediaQuery.addEventListener("change", listener);

        return () => mediaQuery.removeEventListener("change", listener);
    }, [query]);

    return matches;
}

export function ThemeProvider({
    children,
    ...props
}: ThemeProviderProps) {
    const [ theme, setTheme ] = useSettings(s => [s.theme, s.setTheme])

    const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
    const evaluatedTheme = theme === "system" ? (prefersDarkMode ? "dark" : "light") : theme
    useEffect(() => {
        const root = window.document.documentElement;

        root.classList.remove("light", "dark");
        root.classList.add(evaluatedTheme);
    }, [evaluatedTheme]);

    return (
        <ThemeProviderContext.Provider {...props} value={{
            theme,
            evaluatedTheme,
            setTheme,
        }}>
            { children }
        </ThemeProviderContext.Provider>
    );
}

export const useTheme: () => ThemeProviderState = () => {
    const context = useContext(ThemeProviderContext);

    if (context === undefined)
        throw new Error("useTheme must be used within a ThemeProvider");

    return context;
};
