<template>
  <div
      class="min-h-screen bg-gradient-to-b from-slate-50 to-white dark:from-gray-950 dark:to-gray-900 transition-colors">
    <div class="max-w-[1600px] mx-auto px-3 py-4">
      <!-- Header -->
      <Header v-model:is-dark="isDark"/>

      <!-- Main Content -->
      <main class="grid grid-cols-1 lg:grid-cols-5 gap-5">
        <!-- Settings Panel -->
        <div class="lg:col-span-2">
          <div class="sticky top-4">
            <DedupSettings
                v-model:strategy="strategy"
                :is-dark="isDark"
                class="bg-white dark:bg-gray-900 rounded-2xl ring-1 ring-slate-200 dark:ring-gray-800 p-5 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:repeating-linear-gradient(45deg,rgba(0,0,0,0.02)_0px,rgba(0,0,0,0.02)_1px,transparent_1px,transparent_3px),repeating-linear-gradient(-45deg,rgba(0,0,0,0.02)_0px,rgba(0,0,0,0.02)_1px,transparent_1px,transparent_3px)] dark:[background-image:repeating-linear-gradient(45deg,rgba(255,255,255,0.02)_0px,rgba(255,255,255,0.02)_1px,transparent_1px,transparent_3px),repeating-linear-gradient(-45deg,rgba(255,255,255,0.02)_0px,rgba(255,255,255,0.02)_1px,transparent_1px,transparent_3px)] [background-size:4px_4px]"
            />
          </div>
        </div>

        <!-- Editor and Results -->
        <div class="lg:col-span-3 space-y-5">
          <TextEditor
              v-model="text"
              :is-processing="isProcessing"
              @process="findDuplicates"
          />

          <DuplicateResults
              :duplicates="results"
              class="min-h-[300px] bg-white dark:bg-gray-900 rounded-2xl ring-1 ring-slate-200 dark:ring-gray-800 p-5 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:repeating-linear-gradient(0deg,rgba(0,0,0,0.02)_0px,rgba(0,0,0,0.02)_1px,transparent_1px,transparent_2px),repeating-linear-gradient(90deg,rgba(0,0,0,0.02)_0px,rgba(0,0,0,0.02)_1px,transparent_1px,transparent_2px)] dark:[background-image:repeating-linear-gradient(0deg,rgba(255,255,255,0.02)_0px,rgba(255,255,255,0.02)_1px,transparent_1px,transparent_2px),repeating-linear-gradient(90deg,rgba(255,255,255,0.02)_0px,rgba(255,255,255,0.02)_1px,transparent_1px,transparent_2px)] [background-size:3px_3px]"
          />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
  import {ref, watch} from 'vue';
  import {useDark} from '@vueuse/core';

  import DedupSettings from './components/molecules/DedupSettings.vue';
  import DuplicateResults from './components/molecules/DuplicateResults.vue';
  import {invoke} from '@tauri-apps/api/core';
  import Header from './components/molecules/Header.vue';
  import TextEditor from './components/molecules/TextEditor.vue';

  const isDark = useDark();

  interface DedupStrategy {
    similarity_threshold: number;
    case_sensitive: boolean;
    ignore_whitespace: boolean;
    ignore_punctuation: boolean;

    [key: string]: any;
  }

  const strategy = ref<DedupStrategy>({
    similarity_threshold: 0.7,
    case_sensitive: false,
    ignore_whitespace: true,
    ignore_punctuation: true,
  });

  const text = ref('');
  const results = ref<any[]>([]);
  const isProcessing = ref(false);


  async function findDuplicates(editorText: string) {
    isProcessing.value = true;
    try {
      const duplicates = await invoke('find_duplicates', {
        text: editorText,
        strategy: {
          ...strategy.value,
          similarity_threshold: Number(strategy.value.similarity_threshold),
        },
      });
      results.value = duplicates as any[];
    } catch (error) {
      console.error('Error finding duplicates:', error);
    } finally {
      isProcessing.value = false;
    }
  }

  // Watch for changes in text or strategy to auto-update results
  watch([text, strategy], async () => {
    if (text.value.trim()) {
      await findDuplicates(text.value);
    }
  }, {deep: true});
</script>