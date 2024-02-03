import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./main.css";
import { ThemeProvider } from "./components/theme-provider";
import { RouterProvider, createRouter, RoutePaths } from '@tanstack/react-router'

// Import the generated route tree
import { routeTree } from "./routeTree.gen"
export type RoutePath = RoutePaths<typeof routeTree>

// Create a new router instance
const router = createRouter({ routeTree })

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router
  }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
        <RouterProvider router={router} />
    </ThemeProvider>
  </React.StrictMode>,
);
