<template>
  <div class="tree-container">
    <div v-for="item in treeDisplay" :key="item.item.id">
      <div class="tree-item" :style="{ marginLeft: item.indent * 20 + 'px' }">
        <button :disabled="item.item.is_root" @click="selectItems([item.item.id])">
          {{ item.item.name }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { selectItems } from '@/helpers/editor'
import { type FrontendItem } from '/wasm/pkg/pyramus_wasm'
import { computed, type PropType } from 'vue'

const props = defineProps({
  items: {
    type: Object as PropType<Record<number, FrontendItem>>,
    default: null,
  },
  selectedInternalId: {
    type: Number,
    default: null,
  },
})

const root = computed(() => {
  return Object.values(props.items).find((item) => item.is_root)
})

type FlatItem = {
  indent: number
  item: FrontendItem
}

function flatList(InternalId: number, indent: number): FlatItem[] {
  const item = props.items[InternalId]
  const thisItem = { indent, item }
  if (item.children.length == 0) {
    return [thisItem]
  } else {
    return [thisItem, ...item.children.flatMap((child) => flatList(child, indent + 1))]
  }
}

const treeDisplay = computed(() => {
  if (!root.value) {
    return []
  }
  return flatList(root.value.id, 0)
})
</script>

<style lang="scss" scoped>
.tree-container {
  display: flex;
  flex-direction: column;
}

.tree-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  padding: 5px;
}
</style>
