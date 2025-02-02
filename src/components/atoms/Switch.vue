<template>
  <Switch
    :modelValue="modelValue"
    @update:modelValue="(value: boolean) => $emit('update:modelValue', value)"
    :disabled="disabled"
    :class="[
      'relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200 ease-in-out',
      modelValue ? 'bg-indigo-500 dark:bg-indigo-400' : 'bg-gray-200 dark:bg-gray-700',
      disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
    ]"
  >
    <span class="sr-only">{{ label }}</span>
    <span
      :class="[
        'inline-block h-4 w-4 transform rounded-full bg-white dark:bg-gray-900 transition-transform duration-200 ease-in-out',
        modelValue ? 'translate-x-6' : 'translate-x-1'
      ]"
    />
  </Switch>
  <span v-if="label" class="ml-3 text-sm text-gray-700 dark:text-gray-300">{{ label }}</span>
</template>

<script setup lang="ts">
import { Switch } from '@headlessui/vue'

defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },
  disabled: {
    type: Boolean,
    default: false
  },
  label: {
    type: String,
    default: ''
  }
})

defineEmits(['update:modelValue'])
</script>

<style scoped>
/* Remove any custom styles as HeadlessUI Switch uses Tailwind classes */
</style>
