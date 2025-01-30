<script setup lang="ts">
import Text from '../atoms/Text.vue'

interface Props {
  label?: string;
  error?: string;
  required?: boolean;
  helpText?: string;
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  error: '',
  required: false,
  helpText: ''
});
</script>

<template>
  <div class="space-y-1">
    <Text
      v-if="label"
      as="label"
      variant="label"
      size="sm"
      class="block cursor-pointer text-theme-primary"
    >
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </Text>
    
    <slot />
    
    <Text
      v-if="error"
      variant="error"
      size="sm"
      class="text-red-500"
    >
      {{ error }}
    </Text>
    
    <Text
      v-else-if="helpText"
      variant="help"
      size="sm"
      class="text-theme-muted"
    >
      {{ helpText }}
    </Text>
  </div>
</template>
