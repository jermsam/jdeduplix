<script setup lang="ts">
import Text from '../atoms/Text.vue'

interface Props {
  label?: string
  error?: string
}

defineProps<Props>()
</script>

<template>
  <div class="w-full">
    <Text
      v-if="label"
      as="label"
      variant="label"
      size="sm"
      class="mb-1 block"
    >
      {{ label }}
    </Text>
    
    <slot />
    
    <Text
      v-if="error"
      variant="error"
      size="sm"
      class="mt-1"
    >
      {{ error }}
    </Text>
  </div>
</template>
