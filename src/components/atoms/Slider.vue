<template>
  <div class="space-y-2">
    <div class="flex items-center justify-between">
      <label v-if="label" class="text-sm font-medium text-slate-700 dark:text-gray-100">{{ label }}</label>
      <div class="text-xs text-slate-500 dark:text-gray-400">{{ (modelValue * 100).toFixed(0) }}%</div>
    </div>
    <div class="relative h-10 group">
      <!-- Track Background -->
      <div class="absolute top-1/2 -mt-[3px] w-full h-1.5 rounded-full bg-slate-200/80 dark:bg-gray-800/80"></div>
      
      <!-- Active Track -->
      <div 
        class="absolute top-1/2 -mt-[3px] left-0 h-1.5 rounded-full bg-indigo-500 dark:bg-indigo-400"
        :style="{ width: `${modelValue * 100}%` }"
      ></div>
      
      <!-- Thumb -->
      <button
        ref="thumbRef"
        type="button"
        role="slider"
        :aria-valuenow="modelValue"
        :aria-valuemin="0"
        :aria-valuemax="1"
        :aria-valuetext="`${(modelValue * 100).toFixed(0)}%`"
        class="absolute top-1/2 -mt-2.5 -ml-2.5 w-5 h-5 rounded-full bg-white dark:bg-gray-900 shadow-sm ring-2 ring-indigo-500 dark:ring-indigo-400 cursor-pointer transition-all hover:ring-4 hover:ring-opacity-50 focus:outline-none focus:ring-4 focus:ring-indigo-500/50 dark:focus:ring-indigo-400/50"
        :style="{ left: `${modelValue * 100}%` }"
        @keydown.left.prevent="updateValue(Math.max(0, modelValue - 0.01))"
        @keydown.right.prevent="updateValue(Math.min(1, modelValue + 0.01))"
        @keydown.down.prevent="updateValue(Math.max(0, modelValue - 0.01))"
        @keydown.up.prevent="updateValue(Math.min(1, modelValue + 0.01))"
        @keydown.home.prevent="updateValue(0)"
        @keydown.end.prevent="updateValue(1)"
        @keydown.pagedown.prevent="updateValue(Math.max(0, modelValue - 0.1))"
        @keydown.pageup.prevent="updateValue(Math.min(1, modelValue + 0.1))"
      >
        <span class="sr-only">Adjust value</span>
      </button>

      <!-- Click Target -->
      <div 
        class="absolute inset-0 cursor-pointer"
        @mousedown="startDrag"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'

const props = defineProps<{
  modelValue: number
  label?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: number): void
}>()

const thumbRef = ref<HTMLButtonElement | null>(null)
let isDragging = false

function updateValue(value: number) {
  emit('update:modelValue', Math.round(value * 100) / 100)
}

function startDrag(event: MouseEvent) {
  isDragging = true
  updateFromMouseEvent(event)
  
  window.addEventListener('mousemove', onDrag)
  window.addEventListener('mouseup', stopDrag)
}

function stopDrag() {
  isDragging = false
  window.removeEventListener('mousemove', onDrag)
  window.removeEventListener('mouseup', stopDrag)
}

function onDrag(event: MouseEvent) {
  if (!isDragging) return
  updateFromMouseEvent(event)
}

function updateFromMouseEvent(event: MouseEvent) {
  const container = event.currentTarget as HTMLElement
  const rect = container.getBoundingClientRect()
  const x = Math.min(Math.max(0, event.clientX - rect.left), rect.width)
  updateValue(x / rect.width)
}

onMounted(() => {
  // Add touch support
  const thumb = thumbRef.value
  if (!thumb) return

  thumb.addEventListener('touchstart', (e) => {
    e.preventDefault()
    startDrag(e.touches[0] as unknown as MouseEvent)
  })
})

onBeforeUnmount(() => {
  stopDrag()
})
</script>

<style scoped>
.group {
  touch-action: none;
  user-select: none;
}
</style>
