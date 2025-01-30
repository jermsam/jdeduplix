<script setup lang="ts">
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';

interface Props {
  open: boolean;
  title?: string;
  size?: 'sm' | 'md' | 'lg';
}

defineEmits<{
  (e: 'close'): void;
}>();

const props = withDefaults(defineProps<Props>(), {
  open: false,
  title: '',
  size: 'md'
});

const sizeClasses = {
  sm: 'max-w-sm',
  md: 'max-w-md',
  lg: 'max-w-lg'
};
</script>

<template>
  <TransitionRoot appear :show="open" as="template">
    <Dialog as="div" class="relative z-50" @close="$emit('close')">
      <TransitionChild
        as="template"
        enter="duration-300 ease-out"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="duration-200 ease-in"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-black/25 backdrop-blur-sm" />
      </TransitionChild>

      <div class="fixed inset-0 overflow-y-auto">
        <div class="flex min-h-full items-center justify-center p-4 text-center">
          <TransitionChild
            as="template"
            enter="duration-300 ease-out"
            enter-from="opacity-0 scale-95"
            enter-to="opacity-100 scale-100"
            leave="duration-200 ease-in"
            leave-from="opacity-100 scale-100"
            leave-to="opacity-0 scale-95"
          >
            <DialogPanel 
              :class="[
                sizeClasses[size],
                'w-full transform overflow-hidden rounded-lg theme-surface p-6 text-left align-middle shadow-xl transition-all'
              ]"
            >
              <DialogTitle v-if="title" class="text-lg font-medium leading-6 text-theme-primary mb-4">
                {{ title }}
              </DialogTitle>

              <div class="mt-2">
                <slot />
              </div>

              <div v-if="$slots.footer" class="mt-6 flex justify-end gap-3">
                <slot name="footer" />
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>
