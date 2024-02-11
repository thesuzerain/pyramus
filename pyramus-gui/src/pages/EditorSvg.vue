<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

import { testRenderString, getStageObject, deleteItem } from '@/helpers/editor'

const canvasString = ref('')

canvasString.value = testRenderString()

const stageObject = ref(getStageObject())

const removeItem = (id: number) => {
  const stageItem = stageObject.value.items[id]
  deleteItem(stageItem.id)
}

// every 5 seconds update the canvas
// todo: updates should be triggered by the server after command resolution
setInterval(() => {
  canvasString.value = testRenderString()
  stageObject.value = getStageObject()
}, 1000)
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
