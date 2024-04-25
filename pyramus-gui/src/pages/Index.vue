<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useBreadcrumbs } from '@/store/breadcrumbs'
import { testHelloFromRust } from '@/helpers/test'
import { ref } from 'vue'
import RowDisplay from '@/components/ui/RowDisplay.vue'

const route = useRoute()
const breadcrumbs = useBreadcrumbs()
const testString = ref(testHelloFromRust())

breadcrumbs.setRootContext({ name: 'Home', link: route.path })

// Set up rowdisplay
const displayItems = ref([
  {
    label: '1n',
    id: '1n',
    clientWidth: 0,
  }
])

</script>

<template>
  <div class="page-container">
    Hi :)
    {{ testString }}
    <RowDisplay :items="displayItems" label="Test" base-url="editor" />
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
</style>
