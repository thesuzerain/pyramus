<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref } from 'vue'
import { useBreadcrumbs } from '@/store/breadcrumbs'
const route = useRoute()
const breadcrumbs = useBreadcrumbs()

breadcrumbs.setContext({ name: 'EditorSvg', link: route.path })

import { testRenderString } from '@/helpers/editor'

const canvasString = ref('')

canvasString.value = testRenderString()

</script>

<template>
  <div class="page-container">
    <div v-html="canvasString"></div>
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
  image-rendering: crisp-edges
}
.canvas2 {
  // Todo: changeable size
  width: 20rem;
  height: 20rem;
  image-rendering: pixelated; 
  image-rendering: crisp-edges
}
</style>
