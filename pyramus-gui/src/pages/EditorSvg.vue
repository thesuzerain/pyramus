<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

import { testRenderString, getStageObject, deleteItem } from '@/helpers/editor'
import { type UpdateStage, type Rerender, subscribe } from '@/helpers/messages'

const canvasString = ref('')

canvasString.value = testRenderString()

const stageObject = ref(getStageObject())

const removeItem = (id: number) => {
  const stageItem = stageObject.value.items[id]
  deleteItem(stageItem.id)
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
  <div class="page-container">
    <div v-html="canvasString"></div>
  </div>
  <div>
    Object: {{ stageObject }}
    Editor buttons:
    <div v-for="item in stageObject.items" :key="item.id">
      <button @click="removeItem(item.id)">delete {{ item.name }}</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
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
.canvas2 {
  // Todo: changeable size
  width: 20rem;
  height: 20rem;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
}
</style>
