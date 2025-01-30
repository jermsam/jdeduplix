<script setup lang="ts">
import { computed, withDefaults } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'outline';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  type?: 'button' | 'submit' | 'reset';
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
  type: 'button'
});

const variantClasses = {
  primary: 'bg-brand-primary hover:bg-brand-primary/90 text-white shadow-sm',
  secondary: 'bg-theme-bg-elevated hover:bg-theme-bg-elevated/90 text-theme-primary ring-1 ring-theme-border',
  outline: 'bg-transparent hover:bg-theme-bg-elevated text-theme-primary ring-1 ring-theme-border'
};

const sizeClasses = {
  sm: 'px-2.5 py-1.5 text-xs',
  md: 'px-3 py-2 text-sm',
  lg: 'px-4 py-2 text-base'
};

const baseClasses = 'inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brand-primary disabled:opacity-50 disabled:cursor-not-allowed'

const classes = computed(() => {
  return [
    baseClasses,
    variantClasses[props.variant],
    sizeClasses[props.size]
  ].join(' ')
})
</script>

<template>
  <button
    :type="props.type"
    :disabled="props.disabled || props.loading"
    :class="classes"
  >
    <span v-if="props.loading" class="mr-2">
      <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    </span>
    <slot />
  </button>
</template>
