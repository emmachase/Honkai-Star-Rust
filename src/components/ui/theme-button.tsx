import { MoonIcon, SunIcon } from "lucide-react";
import { Button } from "./button";
import { useTheme } from "../theme-provider";
import { useContext } from "react";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "./dropdown-menu";

export function ThemeButton() {
    const { evaluatedTheme, setTheme } = useTheme()

    return (
        <DropdownMenu >
            <DropdownMenuTrigger asChild>
                <Button size="icon" variant="ghost" onClick={() => setTheme("light")}>
                    { evaluatedTheme === "dark"
                        ? <MoonIcon />
                        : <SunIcon /> }
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side="top" align="start" className="w-12">
                <DropdownMenuItem onClick={() => setTheme("light")}>Light</DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("dark")}>Dark</DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("system")}>System</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    )
}
