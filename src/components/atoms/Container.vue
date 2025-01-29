<script setup lang="ts">
interface Props {
  maxWidth?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | 'full'
  padded?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  maxWidth: 'lg',
  padded: true
})

const maxWidthClasses = {
  sm: 'max-w-screen-sm',
  md: 'max-w-screen-md',
  lg: 'max-w-screen-lg',
  xl: 'max-w-screen-xl',
  '2xl': 'max-w-screen-2xl',
  full: 'max-w-full'
}
</script>

<template>
  <div
    :class="[
      'w-full mx-auto',
      maxWidthClasses[props.maxWidth],
      props.padded ? 'px-4 sm:px-6 lg:px-8' : ''
    ]"
  >
    <slot />
  </div>
</template>
