import { defineStore } from 'pinia'
import type { LocationQueryRaw } from 'vue-router'

export type Breadcrumb = {
  name: string
  link?: string
  query?: LocationQueryRaw
}

type BreadcrumbsStore = {
  names: Map<string, string>
  context?: Breadcrumb
  rootContext?: Breadcrumb
}

declare module 'vue-router' {
  // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
  interface RouteMeta {
    breadcrumb: Breadcrumb[];
  }
}

export const useBreadcrumbs = defineStore('breadcrumbsStore', {
  state: (): BreadcrumbsStore => ({
    names: new Map(),
  }),
  actions: {
    getName(route : string) {
      return this.names.get(route) ?? ''
    },
    setName(route  : string, title  : string) {
      this.names.set(route, title)
    },
    // resets breadcrumbs to only included ones as to not have stale breadcrumbs
    resetToNames(breadcrumbs : Breadcrumb[]) {
      // names is an array of every breadcrumb.name that starts with a ?
      const names = breadcrumbs
        .filter((breadcrumb) => breadcrumb.name.charAt(0) === '?')
        .map((breadcrumb) => breadcrumb.name.slice(1))
      // remove all names that are not in the names array
      for (const [route] of this.names) {
        if (!names.includes(route)) {
          this.names.delete(route)
        }
      }
    },
    setContext(context : Breadcrumb) {
      this.context = context
    },
    setRootContext(context : Breadcrumb) {
      this.rootContext = context
    },
  },
})
