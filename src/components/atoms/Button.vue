<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'ghost'
  size?: 'sm' | 'base' | 'lg'
  disabled?: boolean
  type?: 'button' | 'submit'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'base',
  disabled: false,
  type: 'button'
})

const baseClasses = 'inline-flex items-center justify-center rounded-lg font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed'

const variantClasses = {
  primary: 'bg-indigo-600 text-white hover:bg-indigo-700 focus:ring-indigo-500',
  secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-indigo-500 dark:bg-gray-800 dark:text-gray-100 dark:border-gray-600 dark:hover:bg-gray-700',
  ghost: 'text-gray-700 hover:bg-gray-100 focus:ring-gray-500 dark:text-gray-200 dark:hover:bg-gray-800'
}

const sizeClasses = {
  sm: 'px-3 py-1.5 text-sm',
  base: 'px-4 py-2 text-sm',
  lg: 'px-6 py-3 text-base'
}
</script>

<template>
  <button
    :type="type"
    :disabled="disabled"
    :class="[
      baseClasses,
      variantClasses[variant],
      sizeClasses[size]
    ]"
  >
    <slot />
  </button>
</template>
