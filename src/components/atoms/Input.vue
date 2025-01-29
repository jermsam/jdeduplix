<script setup lang="ts">
import FormField from '../molecules/FormField.vue'

interface Props {
  modelValue: string | number
  type?: 'text' | 'number' | 'email' | 'password' | 'search' | 'tel' | 'url'
  placeholder?: string
  label?: string
  disabled?: boolean
  error?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  placeholder: '',
  label: '',
  disabled: false,
  error: ''
})

const emit = defineEmits(['update:modelValue'])

const baseClasses = 'w-full rounded-lg border bg-white px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-gray-800'

const stateClasses = {
  default: 'border-gray-300 focus:border-indigo-500 focus:ring-indigo-500 dark:border-gray-600 dark:text-gray-100',
  error: 'border-red-300 focus:border-red-500 focus:ring-red-500 dark:border-red-600 dark:text-red-100'
}
</script>

<template>
  <FormField
    :label="props.label"
    :error="props.error"
  >
    <input
      :type="props.type"
      :value="props.modelValue"
      :placeholder="props.placeholder"
      :disabled="props.disabled"
      :class="[
        baseClasses,
        props.error ? stateClasses.error : stateClasses.default
      ]"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    >
  </FormField>
</template>
