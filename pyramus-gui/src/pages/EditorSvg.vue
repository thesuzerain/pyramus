<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed, ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
import { getStageObject } from '@/helpers/editor'
import {subscribe } from '@/helpers/messages'
import ItemEditor from '@/components/ui/ItemEditor.vue'
import TreeSelector from '@/components/ui/TreeSelector.vue'
import ItemWindow from '@/components/ui/ItemWindow.vue'

const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

const stageObject = ref(getStageObject())
const selectedItem = computed(() => {
  const selectedIds = stageObject.value.selected
  if (selectedIds.length !== 1) {
    return undefined
  }
  return stageObject.value.items[selectedIds[0]]
})

subscribe('UpdateStage', async () => {
  stageObject.value = getStageObject()
})

</script>

<template>
  <div class="container">
    <div class="page-container">
      <ItemWindow :stage="stageObject" />
      {{ stageObject }}
    </div>
    <div class="tool-container">
      <div class="tool-section">
        <div v-if="selectedItem" class="tools-list">
          <ItemEditor :key="selectedItem.id" :item="selectedItem" />
        </div>
        <div class="tree-view">
          <TreeSelector
            :items="stageObject.items"
            :selected-item-id="selectedItem?.id"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-direction: row;
}

.page-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.tool-container {
  display: flex;
  flex-direction: column;
  align-items: right;
  justify-content: space-between;
  height: 100%;
  background-color: var(--color-raised-bg);
  box-shadow: var(--shadow-inset-sm), var(--shadow-floating);
  padding: var(--gap-md);
}

.tools-list {
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

.tree-view {
  padding: 0.5rem;
}
</style>
