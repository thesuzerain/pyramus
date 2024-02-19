<template>
  <div class="editor-container">
    <div class="tool-row">
      Name:

      <input
        v-model="nameValue"
        type="text"
        placeholder="placeholder"
        class="text-entry-input"
        @input="updateTransform"
      />
    </div>
    <div class="tool-row">
      Rotation:
      <input
        v-model.number="rotationValue"
        type="number"
        class="number-input"
        @input="updateTransform"
      />
      <input v-model="rotationValue" type="range" min="-180" max="180" @input="updateTransform" />
    </div>
    <div class="tool-row">
      Position:
      <input
        v-model.number="positionValue[0]"
        type="number"
        class="number-input"
        @input="updateTransform"
      />
      <input
        v-model.number="positionValue[1]"
        type="number"
        class="number-input"
        @input="updateTransform"
      />
    </div>
    <div class="tool-row">
      Scale:
      <input
        v-model.number="scaleValue[0]"
        type="number"
        class="number-input"
        @input="updateTransform"
      />
      <input
        v-model.number="scaleValue[1]"
        type="number"
        class="number-input"
        @input="updateTransform"
      />
    </div>
    <Button @click="removeItem">Delete</Button>
  </div>
</template>

<script setup lang="ts">
import { type FrontendItem } from '/wasm/pkg/pyramus_wasm'
import { ref, type PropType } from 'vue'
import { deleteItem, editItemTransform } from '@/helpers/editor'

import Button from '@/components/ui/Button.vue'

const props = defineProps({
  item: {
    type: Object as PropType<FrontendItem>,
    default: null,
  },
})

const rotationValue = ref(props.item.rotation)
const nameValue = ref(props.item.name)
const positionValue = ref(props.item.position)
const scaleValue = ref(props.item.scale)

const updateTransform = () => {
  console.log('updateTransform')
  editItemTransform(props.item.id, positionValue.value, rotationValue.value, scaleValue.value)
}

const removeItem = () => {
  deleteItem(props.item.id)
}
</script>

<style lang="scss" scoped>
.editor-container {
  display: flex;
  flex-direction: column;
}

.tool-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  margin: 5px;
}

.number-input {
  width: 40px;
  margin: 5px;
}

.text-entry-input {
  margin: 5px;
}
</style>
