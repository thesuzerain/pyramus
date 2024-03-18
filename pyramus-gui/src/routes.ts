import { createRouter, createWebHistory } from 'vue-router'
import * as Pages from '@/pages'

/**
 * Configures application routing. Add page to pages/index and then add to route table here.
 */
const routes = [
  {
    path: '/',
    name: 'Home',
    component: Pages.Index,
    meta: {
      breadcrumb: [{ name: 'Home' }],
    },
  },
  {
    path: '/editor/prop',
    name: 'Prop Editor',
    component: Pages.EditorSvg,
    meta: {
      breadcrumb: [{ name: 'Prop Editor' }],
    },
  },
  {
    path: '/editor/blueprint',
    name: 'Blueprint Editor',
    component: Pages.EditorSvg,
    meta: {
      breadcrumb: [{ name: 'Blueprint Editor' }],
    },
  },
]

export default createRouter({
  history: createWebHistory(),
  routes,
  linkActiveClass: 'router-link-active',
  linkExactActiveClass: 'router-link-exact-active',
  scrollBehavior() {
    // always scroll to top
    return { top: 0 }
  },
})
