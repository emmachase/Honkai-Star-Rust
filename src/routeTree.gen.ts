// This file is auto-generated by TanStack Router

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as RelicsImport } from './routes/relics'
import { Route as CharactersImport } from './routes/characters'
import { Route as SplatImport } from './routes/$'
import { Route as IndexImport } from './routes/index'

// Create/Update Routes

const RelicsRoute = RelicsImport.update({
  path: '/relics',
  getParentRoute: () => rootRoute,
} as any)

const CharactersRoute = CharactersImport.update({
  path: '/characters',
  getParentRoute: () => rootRoute,
} as any)

const SplatRoute = SplatImport.update({
  path: '/$',
  getParentRoute: () => rootRoute,
} as any)

const IndexRoute = IndexImport.update({
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/$': {
      preLoaderRoute: typeof SplatImport
      parentRoute: typeof rootRoute
    }
    '/characters': {
      preLoaderRoute: typeof CharactersImport
      parentRoute: typeof rootRoute
    }
    '/relics': {
      preLoaderRoute: typeof RelicsImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  IndexRoute,
  SplatRoute,
  CharactersRoute,
  RelicsRoute,
])
