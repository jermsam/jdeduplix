<template>
  <label class="switch-wrapper" :class="{ disabled }">
    <input
      type="checkbox"
      :checked="modelValue"
      :disabled="disabled"
      @change="$emit('update:modelValue', $event.target.checked)"
    >
    <span class="switch">
      <span class="switch-handle"></span>
    </span>
    <span v-if="label" class="switch-label">{{ label }}</span>
  </label>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: boolean
  label?: string
  disabled?: boolean
}>()

defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()
</script>

<style scoped>
.switch-wrapper {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.switch-wrapper.disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.switch {
  position: relative;
  display: inline-block;
  width: 36px;
  height: 20px;
  background-color: #e0e0e0;
  border-radius: 10px;
  transition: background-color 0.2s;
}

input:checked + .switch {
  background-color: #4CAF50;
}

.switch-handle {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

input:checked + .switch .switch-handle {
  transform: translateX(16px);
}

input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-label {
  font-size: 0.875rem;
  color: #333;
  user-select: none;
}

/* Hover effects */
.switch-wrapper:not(.disabled):hover .switch {
  background-color: #d0d0d0;
}

.switch-wrapper:not(.disabled):hover input:checked + .switch {
  background-color: #43A047;
}
</style>
