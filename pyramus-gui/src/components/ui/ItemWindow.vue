<template>
  <div class="window-container">
    <div
      ref="clickableDiv"
      @mousedown="mouseDown"
      @mouseup="mouseUp"
      @mousemove="mouseMove"
      v-html="canvasString"
    ></div>
  </div>
  {{ props.stage.selected }}
</template>

<script setup lang="ts">
import { type FrontendStage } from '/wasm/pkg/pyramus_wasm'
import { ref, type PropType } from 'vue'
import { testRenderString } from '@/helpers/editor'
import { subscribe } from '@/helpers/messages'
import { handleMouseMove, handleMouseDown, handleMouseUp } from '@/helpers/input'

const props = defineProps({
  stage: {
    type: Object as PropType<FrontendStage>,
    default: null,
  },
})

const clickableDiv = ref<HTMLElement | undefined>(undefined)
const canvasString = ref('')
canvasString.value = testRenderString()

const mouseMove = (event: MouseEvent) => {
  if (!clickableDiv.value) return
  handleMouseMove(event.movementX, event.movementY)
}

const mouseDown = (event: MouseEvent) => {
  if (!clickableDiv.value) return
  const rect = clickableDiv.value.getBoundingClientRect()
  const x = event.clientX - rect.left
  const y = event.clientY - rect.top
  handleMouseDown(x, y)
}

const mouseUp = () => {
  if (!clickableDiv.value) return
  handleMouseUp()
}

subscribe('Rerender', async () => {
  canvasString.value = testRenderString()
})
</script>

<style lang="scss" scoped></style>
