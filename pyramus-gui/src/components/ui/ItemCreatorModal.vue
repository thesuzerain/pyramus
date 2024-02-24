<template>
  <Modal ref="modal" :header="`Create new item in ${parent?.name}`">
    <div class="modal-body">
      <div class="button-row">
        <Button :disabled="itemType == ItemType.Image" @click="itemType = ItemType.Image"
          >Image</Button
        >
        <Button :disabled="itemType == ItemType.SVG" @click="itemType = ItemType.SVG">SVG</Button>
        <Button :disabled="itemType == ItemType.Text" @click="itemType = ItemType.Text"
          >Text</Button
        >
      </div>
      <div>
        <div>Parent: {{ parent?.name }}</div>
        <input v-model="nameValue" type="text" placeholder="Item name" class="text-entry-input" />
      </div>
      <div v-if="itemType == ItemType.Image" class="image-creation">
        <input type="file" @change="handleFileChange" />
      </div>
      <div v-if="itemType == ItemType.SVG" class="svg-creation">
        <textarea
          v-model="svgValue"
          placeholder="Input SVG string here..."
          class="long-text-entry-input"
        />
        <div v-html="svgValue"></div>
      </div>
      <div v-if="itemType == ItemType.Text" class="text-creation">
        <textarea
          v-model="textValue"
          placeholder="Input text here..."
          class="long-text-entry-input"
        />
      </div>
      <button :disabled="!canSubmit" @click="submit">Create</button>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import Modal from '@/components/ui/Modal.vue'
import { createImage, createSvg, createText } from '@/helpers/uploadImage'
import { type FrontendItem } from '/wasm/pkg/pyramus_wasm'
import Button from './Button.vue'

enum ItemType {
  Image,
  SVG,
  Text,
}
const itemType = ref(ItemType.SVG)

const modal = ref<typeof Modal | undefined>(undefined)

const nameValue = ref('')
const svgValue = ref('')
const fileValue = ref<File | undefined>(undefined)
const textValue = ref('')
const parent = ref<FrontendItem | undefined>(undefined)

function submit() {
  if (!parent.value) {
    return
  }
  switch (itemType.value) {
    case ItemType.Image:
      if (!fileValue.value) {
        return
      }
      createImage(nameValue.value, parent.value.id, fileValue.value)
      break
    case ItemType.SVG:
      createSvg(nameValue.value, parent.value.id, svgValue.value)
      break
    case ItemType.Text:
      createText(nameValue.value, parent.value.id, textValue.value)
      break
  }

  modal.value?.hide()
}

const canSubmit = computed(() => {
  if (!parent.value) {
    return false
  }
  if (nameValue.value.length == 0) {
    return false
  }
  switch (itemType.value) {
    case ItemType.Image:
      return fileValue.value != undefined
    case ItemType.SVG:
      return svgValue.value != ''
    case ItemType.Text:
      return true
  }
  return false
})

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  console.log(file)
  if (file) {
    fileValue.value = file
  } else {
    fileValue.value = undefined
  }
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
