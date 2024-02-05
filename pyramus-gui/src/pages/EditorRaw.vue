<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorRaw', link: route.path })

import { clearCanvas, testRender } from '@/helpers/editor'

const canvas = ref(null)

onMounted(async () => {
  if (canvas.value) {
    clearCanvas(canvas.value) // todo: unneeded?
    testRender(canvas.value)
  }
})
</script>

<template>
  <div class="page-container">
    <canvas id="canvas" ref="canvas" class="canvas" width="800" height="600"></canvas>
  </div>
</template>

<style lang="scss" scoped>
.page-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
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
