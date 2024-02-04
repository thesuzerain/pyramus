import { createRouter, createWebHistory } from 'vue-router'
import * as Pages from '@/pages'

/**
 * Configures application routing. Add page to pages/index and then add to route table here.
 */
export default new createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Pages.Index,
      meta: {
        breadcrumb: [{ name: 'Home' }],
      },
    },
    {
      path: '/editor/raw',
      name: 'Editor Raw',
      component: Pages.EditorRaw,
      meta: {
        breadcrumb: [{ name: 'Editor Raw' }]
      }
    },
    {
      path: '/editor/svg',
      name: 'Editor SVG',
      component: Pages.EditorSvg,
      meta: {
        breadcrumb: [{ name: 'Editor SVG' }]
      }
    },


  ],
  linkActiveClass: 'router-link-active',
  linkExactActiveClass: 'router-link-exact-active',
  scrollBehavior() {
    // always scroll to top
    return { top: 0 }
  },
})
