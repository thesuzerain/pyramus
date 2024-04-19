<template>
  <Modal ref="modal" :header="`Export stage`">
    <div class="modal-body">
      <div class="button-row">
        <Button :disabled="itemType == ItemType.SVG" @click="itemType = ItemType.SVG">SVG</Button>
        <Button :disabled="itemType == ItemType.JSON" @click="itemType = ItemType.JSON"
          >JSON</Button
        >
      </div>
      <div v-if="itemType == ItemType.SVG" class="svg-display">
        <button @click="copyToClipboard(svgValue)">Copy to Clipboard</button>
        <textarea v-model="svgValue" class="long-text-entry-input" readonly />
      </div>
      <div v-if="itemType == ItemType.JSON" class="text-creation">
        <button @click="copyToClipboard(jsonValue)">Copy to Clipboard</button>
        <textarea v-model="jsonValue" class="long-text-entry-input" readonly />
      </div>
      <!-- <button :disabled="!canSubmit" @click="submit">Create</button> -->
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Modal from '@/components/ui/Modal.vue'
import { type FrontendItem } from '/wasm/pkg/pyramus_wasm'
import Button from './Button.vue'
import { testRenderString, testJsonString } from '@/helpers/editor'

enum ItemType {
  SVG,
  JSON,
}
const itemType = ref(ItemType.SVG)

const modal = ref<typeof Modal | undefined>(undefined)

const svgValue = ref(testRenderString())
const jsonValue = ref(testJsonString())

const parent = ref<FrontendItem | undefined>(undefined)

// TODO: can  make a component for reusable clipboard copying
function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text)
}

defineExpose({
  show: (itemParent: FrontendItem) => {
    parent.value = itemParent
    modal.value?.show()
  },
})
</script>

<style lang="scss" scoped>
.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
}

.button-row {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
}

.text-entry-input {
  margin: 5px;
}

.long-text-entry-input {
  width: 100%;
  height: 100px;
}
</style>
