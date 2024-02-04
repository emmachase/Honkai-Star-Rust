import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { ThemeButton } from "@/components/ui/theme-button";
import { Column, Row } from "@/components/util/flex";
import { AnyRoute, createRootRoute, Link, LinkProps, Outlet, RegisteredRouter, RoutePaths, ToOptions, useMatch, useMatches, useMatchRoute } from "@tanstack/react-router";
import { MoonIcon, SettingsIcon } from "lucide-react";
import React from "react";

function NavLink<
    TRouteTree extends AnyRoute = RegisteredRouter['routeTree'],
    TFrom extends RoutePaths<TRouteTree> | string = string,
    TTo extends string = '',
>(props: LinkProps<TRouteTree, TFrom, TTo> & React.RefAttributes<HTMLAnchorElement>) {
    const active = useMatchRoute()({ to: props.to as string })

    return (
        <Button
            size="sm"
            variant={active ? "default" : "ghost"}
            className="justify-start"
            asChild
        >
            <Link {...props}/>
        </Button>
    );
}

function Nav() {
    return (
        <Column className="bg-card rounded-md border">
            <Column className="p-2 gap-2 flex-1">
                <NavLink to="/">Optimizer</NavLink>
                <NavLink to="/characters">Characters</NavLink>
                <NavLink to="/relics">Relics</NavLink>
                <Separator />
                <NavLink to="/import">Import</NavLink>
            </Column>
            <Row className="p-2 justify-between">
                <ThemeButton />

                <Button size="icon" variant="ghost">
                    <SettingsIcon />
                </Button>
            </Row>
        </Column>
    );
}

function Root() {
    return (<>
        <div className="grid p-2 grid-cols-[160px_1fr] h-dvh gap-2">
            <Nav />
            <Outlet />
        </div>
        {/* <TanStackRouterDevtools /> */}
    </>);
}

export const Route = createRootRoute({
    component: Root
});
