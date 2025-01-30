<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary'
}

const props = defineProps<Props>()

const baseClasses = 'inline-flex items-center justify-center rounded-lg font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed'

const variantClasses = computed(() => {
  return props.variant === 'primary' ? 'bg-indigo-500 hover:bg-indigo-600 text-white' : 'bg-[#1E2128] hover:bg-[#23262E] text-gray-300'
})

const classes = computed(() => {
  return [
    baseClasses,
    'px-3 py-1.5 text-xs',
    variantClasses.value
  ].join(' ')
})
</script>

<template>
  <button
    :class="classes"
  >
    <slot />
  </button>
</template>
