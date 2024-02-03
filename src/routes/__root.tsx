import { Button } from "@/components/ui/button";
import { Column } from "@/components/util/flex";
import { AnyRoute, createRootRoute, Link, LinkProps, Outlet, RegisteredRouter, RoutePaths, ToOptions, useMatch, useMatches, useMatchRoute } from "@tanstack/react-router";
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
        <Column className="p-2 gap-2">
            <NavLink to="/">Optimizer</NavLink>
            <NavLink to="/characters">Characters</NavLink>
            <NavLink to="/relics">Relics</NavLink>
        </Column>
    );
}

function Root() {
    return (<>
        <div className="grid p-2 grid-cols-[160px_1fr]">
            <Nav />
            <Outlet />
        </div>
        {/* <TanStackRouterDevtools /> */}
    </>);
}

export const Route = createRootRoute({
    component: Root
});
