<script setup lang="ts">
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';

interface Props {
  open: boolean;
  title?: string;
  size?: 'sm' | 'md' | 'lg';
}

defineEmits<{
  'close': [[], [boolean]];
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
    <Dialog :open="open" as="div" class="relative z-50" >
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-black/25 backdrop-blur-sm" />
      </TransitionChild>

      <div class="fixed inset-0 overflow-y-auto overflow-x-hidden">
        <div class="flex min-h-full items-center justify-center p-4 text-center">
          <TransitionChild
            as="template"
            enter="ease-out duration-300"
            enter-from="opacity-0 scale-95"
            enter-to="opacity-100 scale-100"
            leave="ease-in duration-200"
            leave-from="opacity-100 scale-100"
            leave-to="opacity-0 scale-95"
          >
            <DialogPanel 
              :class="[
                sizeClasses[size],
                'w-full relative theme-surface p-6 rounded-2xl text-left align-middle shadow-xl transition-all max-w-full overflow-hidden'
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
