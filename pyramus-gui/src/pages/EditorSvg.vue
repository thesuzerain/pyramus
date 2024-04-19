<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed, ref, watch } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
import { switchToPropEditor, switchToBlueprintEditor } from '@/helpers/state'
import { getStageObject } from '@/helpers/editor'
import { subscribe } from '@/helpers/messages'
import ItemEditor from '@/components/ui/ItemEditor.vue'
import TreeSelector from '@/components/ui/TreeSelector.vue'
import ItemWindow from '@/components/ui/ItemWindow.vue'
import ItemCreatorModal from '@/components/ui/ItemCreatorModal.vue'
import StageSaveModal from '@/components/ui/StageSaveModal.vue'

const router = useRouter()
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

const stageObject = ref(getStageObject())

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

// Set the editor on page load
// TODO: any
const loadEditor = (type: any) => {
  if (type === 'prop') {
    switchToPropEditor()
  } else if (type === 'blueprint') {
    switchToBlueprintEditor()
  } else {
    // Return to /
    router.push('/')
    return
  }
}

// Call the function immediately on setup for initialization
loadEditor(route.params.type)

// Setup a watcher that reacts whenever the `type` route parameter changes
watch(
  () => route.params.type,
  (newType) => {
    loadEditor(newType)
  }
)

console.log('EditorSvg', route.params)

const itemCreatorModal = ref<typeof ItemCreatorModal | undefined>(undefined)
const stageSaveModal = ref<typeof StageSaveModal | undefined>(undefined)

const selectedItem = computed(() => {
  const selectedIds = stageObject.value.selected
  if (selectedIds.length !== 1) {
    return undefined
  }
  return stageObject.value.items[selectedIds[0]]
})

const rootItem = computed(() => {
  return Object.values(stageObject.value.items).find((item) => item.is_root)
})

subscribe('UpdateStage', async () => {
  stageObject.value = getStageObject()
})
</script>

<template>
  <div class="container">
    <div class="page-container">
      <ItemWindow :stage="stageObject" />
    </div>
    <div class="tool-container">
      <div class="tool-section">
        <div v-if="selectedItem" class="tools-list">
          <ItemEditor
            :key="selectedItem.id"
            :item="selectedItem"
            :create-item-modal="itemCreatorModal"
          />
        </div>
        <div class="tree-view">
          <TreeSelector :items="stageObject.items" :selected-item-id="selectedItem?.id" />
        </div>
        <button @click="itemCreatorModal?.show(rootItem)">Create item</button>
        <button @click="stageSaveModal?.show(stageObject)">Save stage</button>
      </div>
    </div>
  </div>
  <ItemCreatorModal ref="itemCreatorModal" />
  <StageSaveModal ref="stageSaveModal" />
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
  width: 100%; // TODO: When canvas is resizable, this should be smaller
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
