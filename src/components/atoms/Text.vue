<script setup lang="ts">
interface Props {
  as?: 'p' | 'span' | 'div' | 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6'
  size?: 'xs' | 'sm' | 'base' | 'lg' | 'xl' | '2xl'
  weight?: 'normal' | 'medium' | 'semibold' | 'bold'
  leading?: 'none' | 'tight' | 'normal' | 'relaxed'
}

const props = withDefaults(defineProps<Props>(), {
  as: 'p',
  size: 'base',
  weight: 'normal',
  leading: 'normal'
})

const sizeClasses = {
  xs: 'text-xs',
  sm: 'text-sm',
  base: 'text-base',
  lg: 'text-lg',
  xl: 'text-xl',
  '2xl': 'text-2xl'
}

const weightClasses = {
  normal: 'font-normal',
  medium: 'font-medium',
  semibold: 'font-semibold',
  bold: 'font-bold'
}

const leadingClasses = {
  none: 'leading-none',
  tight: 'leading-tight',
  normal: 'leading-normal',
  relaxed: 'leading-relaxed'
}
</script>

<template>
  <component
    :is="props.as"
    :class="[
      sizeClasses[props.size],
      weightClasses[props.weight],
      leadingClasses[props.leading],
      'text-gray-900 dark:text-gray-100'
    ]"
  >
    <slot />
  </component>
</template>
