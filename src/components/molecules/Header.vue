
<template>
  <header class="mb-6">
    <nav class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <div class="relative inline-flex items-center">
          <h1 class="text-2xl sm:text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-500 to-purple-600 dark:from-indigo-400 dark:to-purple-500">Jdeduplix</h1>
          <span class="ml-2 px-1.5 py-0.5 text-[10px] font-semibold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 rounded-full">Alpha</span>
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <button
            class="p-2 rounded-full text-slate-500 hover:text-slate-600 hover:bg-white dark:text-gray-400 dark:hover:text-gray-300 dark:hover:bg-gray-800 ring-1 ring-slate-200 dark:ring-gray-800 transition-all duration-200"
            @click="$emit('update:isDark',!isDark)"
        >
          <span class="sr-only">Toggle dark mode</span>
          <svg
              v-if="isDark"
              class="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
          >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
            />
          </svg>
          <svg
              v-else
              class="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
          >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
            />
          </svg>
        </button>
      </div>
    </nav>
  </header>
</template>

<script setup lang="ts">
  defineProps({
    isDark: Boolean,
  })

  defineEmits(['update:isDark'])

</script>

<style scoped>

</style>