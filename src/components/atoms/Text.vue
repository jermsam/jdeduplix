<script setup lang="ts">
interface Props {
  as?: 'p' | 'span' | 'div' | 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6'
  size?: 'xs' | 'sm' | 'base' | 'lg' | 'xl' | '2xl'
  weight?: 'normal' | 'medium' | 'semibold' | 'bold'
  color?: 'default' | 'muted' | 'white'
}

const props = withDefaults(defineProps<Props>(), {
  as: 'p',
  size: 'base',
  weight: 'normal',
  color: 'default'
})

const sizeClasses = {
  xs: 'text-xs leading-4',
  sm: 'text-sm leading-5',
  base: 'text-base leading-6',
  lg: 'text-lg leading-7',
  xl: 'text-xl leading-7',
  '2xl': 'text-2xl leading-8'
}

const weightClasses = {
  normal: 'font-normal',
  medium: 'font-medium',
  semibold: 'font-semibold',
  bold: 'font-bold'
}

const colorClasses = {
  default: 'text-gray-900 dark:text-gray-100',
  muted: 'text-gray-500 dark:text-gray-400',
  white: 'text-white'
}
</script>

<template>
  <component
    :is="props.as"
    :class="[
      'font-sans',
      sizeClasses[props.size],
      weightClasses[props.weight],
      colorClasses[props.color]
    ]"
  >
    <slot />
  </component>
</template>
