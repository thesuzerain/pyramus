<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
import { testRenderString, getStageObject } from '@/helpers/editor'
import { type UpdateStage, type Rerender, subscribe } from '@/helpers/messages'
import ItemEditor from '@/components/ui/ItemEditor.vue'
import TreeSelector from '@/components/ui/TreeSelector.vue'
import { type FrontendItem } from '/wasm/pkg/pyramus_wasm'

const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

const canvasString = ref('')
const stageObject = ref(getStageObject())

canvasString.value = testRenderString()

const selectedItem = ref<FrontendItem | undefined>(undefined)
function updateSelectedItem(itemId: number) {
  selectedItem.value = stageObject.value.items[itemId]
  console.log('selectedItem', selectedItem.value)
}

subscribe('UpdateStage', async (data: UpdateStage) => {
  console.log('UpdateStage', data)
  stageObject.value = getStageObject()
})

subscribe('Rerender', async (data: Rerender) => {
  console.log('RenderImage', data)
  canvasString.value = testRenderString()
})
</script>

<template>
  <div class="container">
    <div class="page-container">
      <div v-html="canvasString"></div>
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
            @update-value="updateSelectedItem"
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
.canvas {
  // Todo: changeable size
  width: 10rem;
  height: 10rem;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
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
