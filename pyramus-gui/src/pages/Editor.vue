<script setup>
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'Editor', link: route.path })

import { drawCheckerBoard, clearCanvas } from '@/helpers/editor'

const canvas = ref(null)

onMounted(async () => {
  clearCanvas(canvas.value)

  await drawCheckerBoard(canvas.value);
  setInterval(async () => {
    await drawCheckerBoard(canvas.value);
  }, 1000);
})

</script>

<template>
  <div class="page-container">
    <canvas id="canvas" ref="canvas" class="canvas"   width="20" height="20" ></canvas>
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
  width: 25rem;
  height: 25rem;
  image-rendering: pixelated; 
  image-rendering: crisp-edges
}
</style>
