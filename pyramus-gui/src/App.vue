<script setup lang="ts">
import { ref } from 'vue'
import { RouterView, RouterLink } from 'vue-router'
import Button from '@/components/ui/Button.vue'
import Breadcrumbs from '@/components/ui/Breadcrumbs.vue'
import SplashScreen from '@/components/ui/SplashScreen.vue'
import { useLoading, useTheming } from '@/store/state'
import { PlusIcon } from '@/assets/icons'

const themeStore = useTheming()
const isLoading = ref(true)
const loading = useLoading()

defineExpose({
  initialize: async () => {
    isLoading.value = false
    themeStore.setThemeState('dark')
  },
  failure: async (e: string) => {
    isLoading.value = false
    console.error(e)
  },
})

document.querySelector('body')?.addEventListener('auxclick', function (e) {
  // disables middle click -> new tab
  if (e.button === 1) {
    e.preventDefault()
    // instead do a left click
    const event = new MouseEvent('click', {
      view: window,
      bubbles: true,
      cancelable: true,
    })
    if (e.target) e.target.dispatchEvent(event)
  }
})
</script>

<template>
  <SplashScreen v-if="isLoading" app-loading />
  <div v-else class="container">
    <div class="nav-container">
      <div class="nav-section">
        <div class="pages-list">
          <RouterLink v-tooltip="'Home'" to="/" class="btn icon-only collapsed-button">
            <PlusIcon />
          </RouterLink>
          <RouterLink
            v-tooltip="'Editor'"
            to="/editor/1n" 
            class="btn icon-only collapsed-button"
          >
            <PlusIcon />
          </RouterLink>
        </div>
      </div>
      <div class="settings pages-list">
        <Button
          v-tooltip="'Create button'"
          class="sleek-primary collapsed-button"
          icon-only
          @click="() => console.log('You clicked the plus button!')"
        >
          <PlusIcon />
        </Button>
      </div>
    </div>

    <div class="view">
      <div class="appbar-row">
        <div class="appbar">
          <section class="navigation-controls">
            <Breadcrumbs />
          </section>
        </div>
      </div>
      <div class="router-view">
        <RouterView v-slot="{ Component }">
          <template v-if="Component">
            <Suspense @pending="loading.startLoading()" @resolve="loading.stopLoading()">
              <component :is="Component"></component>
            </Suspense>
          </template>
        </RouterView>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sleek-primary {
  background-color: var(--color-brand-highlight);
  transition: all ease-in-out 0.1s;
}

.navigation-controls {
  flex-grow: 1;
  width: min-content;
}

.appbar-row {
  display: flex;
  flex-direction: row;
}

.container {
  --appbar-height: 3.25rem;
  --sidebar-width: 4.5rem;

  height: 100vh;
  display: flex;
  flex-direction: row;
  overflow: hidden;

  .view {
    width: calc(100% - var(--sidebar-width));
    background-color: var(--color-raised-bg);

    .appbar {
      display: flex;
      align-items: center;
      flex-grow: 1;
      background: var(--color-raised-bg);
      text-align: center;
      padding: var(--gap-md);
      height: 3.25rem;
      gap: var(--gap-sm);
      //no select
      user-select: none;
      -webkit-user-select: none;
    }

    .router-view {
      width: 100%;
      height: calc(100% - 3.125rem);
      overflow: auto;
      overflow-x: hidden;
      background-color: var(--color-bg);
      border-top-left-radius: var(--radius-xl);
    }
  }
}

.nav-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  background-color: var(--color-raised-bg);
  box-shadow: var(--shadow-inset-sm), var(--shadow-floating);
  padding: var(--gap-md);
}

.pages-list {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  justify-content: flex-start;
  width: 100%;
  gap: 0.5rem;

  a {
    display: flex;
    align-items: center;
    word-spacing: 3px;
    background: inherit;
    transition: all ease-in-out 0.1s;
    color: var(--color-base);
    box-shadow: none;

    &.router-link-active {
      color: var(--color-contrast);
      background: var(--color-button-bg);
      box-shadow: var(--shadow-floating);
    }

    &:hover {
      background-color: var(--color-button-bg);
      color: var(--color-contrast);
      box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
      text-decoration: none;
    }
  }
}

.collapsed-button {
  height: 3rem !important;
  width: 3rem !important;
  padding: 0.75rem;
  border-radius: var(--radius-md);
  box-shadow: none;

  svg {
    width: 1.5rem !important;
    height: 1.5rem !important;
    max-width: 1.5rem !important;
    max-height: 1.5rem !important;
  }
}

.nav-section {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  width: 100%;
  height: 100%;
  gap: 1rem;
}
</style>
