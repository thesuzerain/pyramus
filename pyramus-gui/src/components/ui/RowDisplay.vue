<!-- Original credit: https://github.com/modrinth/theseus/master/theseus_gui/src/components/RowDisplay.vue -->
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { PlusIcon } from '@/assets/icons';

type DisplayItem = {
  label: string
  id: string
  clientWidth: number
}

const props = defineProps({
  items: {
    type: Array as () => DisplayItem[],
    default() {
      return []
    },
  },
  label: {
    type: String,
    default: '',
  },
  // base for each link route
  // {baseUrl}/{item.id}
  baseUrl: {
    type: String,
    default: '',
  },
  canPaginate: Boolean,
})

const getRoute = (id: string) => {
  return `${props.baseUrl}/${id}`
}

const maxItemsPerRow = ref(1)

const calculateCardsPerRow = () => {
  if (!props.items || !props.items[0]) {
    return
  }

  // Calculate how many cards fit in one row
  const containerWidth = props.items[0].clientWidth
  // Convert container width from pixels to rem
  const containerWidthInRem =
    containerWidth / parseFloat(getComputedStyle(document.documentElement).fontSize)
  maxItemsPerRow.value = Math.floor((containerWidthInRem + 1) / 11)
}

onMounted(() => {
  calculateCardsPerRow()
  window.addEventListener('resize', calculateCardsPerRow)
})

onUnmounted(() => {
  window.removeEventListener('resize', calculateCardsPerRow)
})
</script>

<template>
  <div class="content">
    <div v-for="row in props.items" ref="rows" :key="row.id" class="row">
      <div class="header">
        <router-link :to="getRoute(row.id)">{{ row.label }}</router-link>
        <PlusIcon />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 1rem;
  gap: 1rem;

  -ms-overflow-style: none;
  scrollbar-width: none;

  &::-webkit-scrollbar {
    width: 0;
    background: transparent;
  }
}

.row {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  overflow: hidden;
  width: 100%;
  min-width: 100%;

  &:nth-child(even) {
    background: var(--color-bg);
  }

  .header {
    width: 100%;
    margin-bottom: 1rem;
    gap: var(--gap-xs);
    display: flex;
    flex-direction: row;
    align-items: center;

    a {
      margin: 0;
      font-size: var(--font-size-lg);
      font-weight: bolder;
      white-space: nowrap;
      color: var(--color-contrast);
    }

    svg {
      height: 1.5rem;
      width: 1.5rem;
      color: var(--color-contrast);
    }
  }

  .items {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
    grid-gap: 1rem;
    width: 100%;
  }
}
</style>