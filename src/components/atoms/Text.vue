<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'muted';
  size?: 'xs' | 'sm' | 'base' | 'lg';
  weight?: 'normal' | 'medium' | 'semibold';
  as?: string;
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'base',
  weight: 'normal',
  as: 'p'
});

const Component = props.as;

const variantClasses = {
  primary: 'text-theme-primary',
  secondary: 'text-theme-secondary',
  muted: 'text-theme-muted'
};

const sizeClasses = {
  xs: 'text-xs',
  sm: 'text-sm',
  base: 'text-base',
  lg: 'text-lg'
};

const weightClasses = {
  normal: 'font-normal',
  medium: 'font-medium',
  semibold: 'font-semibold'
};
</script>

<template>
  <Component
    :class="[
      variantClasses[props.variant],
      sizeClasses[props.size],
      weightClasses[props.weight]
    ]"
  >
    <slot />
  </Component>
</template>
