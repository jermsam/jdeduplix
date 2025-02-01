<template>
  <div class="min-h-screen bg-gradient-to-b from-slate-50 to-white dark:from-gray-950 dark:to-gray-900 transition-colors">
    <div class="max-w-[1600px] mx-auto px-3 py-4">
      <!-- Header -->
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
              @click="isDark = !isDark"
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
          <div class="relative group">
            <div class="absolute inset-0 bg-gradient-to-r from-indigo-500/10 to-purple-500/10 dark:from-indigo-500/5 dark:to-purple-500/5 rounded-2xl blur-2xl transition-opacity duration-500 opacity-0 group-hover:opacity-100"></div>
            <div class="relative">
              <div class="absolute top-3 inset-x-3 flex items-center justify-between z-10">
                <div class="flex items-center space-x-3">
                  <div class="flex space-x-1.5">
                    <div class="w-3 h-3 rounded-full bg-red-400/80 dark:bg-red-500/80" />
                    <div class="w-3 h-3 rounded-full bg-amber-400/80 dark:bg-amber-500/80" />
                    <div class="w-3 h-3 rounded-full bg-green-400/80 dark:bg-green-500/80" />
                  </div>
                  <span class="text-sm font-medium text-slate-400 dark:text-slate-500">Editor</span>
                </div>
                <div class="flex items-center space-x-2">
                  <button
                    @click="addTestText"
                    class="px-2 py-1 text-xs font-medium text-slate-600 dark:text-slate-300 hover:text-indigo-500 dark:hover:text-indigo-400 transition-colors"
                  >
                    Add Test Text
                  </button>
                  <button
                    @click="findDuplicates"
                    :disabled="!hasText"
                    class="px-3 py-1 text-xs font-medium text-white bg-gradient-to-r from-indigo-500 to-purple-500 dark:from-indigo-400 dark:to-purple-400 hover:from-indigo-600 hover:to-purple-600 dark:hover:from-indigo-500 dark:hover:to-purple-500 disabled:opacity-50 disabled:cursor-not-allowed rounded-full shadow-sm shadow-indigo-500/20 dark:shadow-indigo-400/20 transition-all hover:shadow-md hover:shadow-indigo-500/25 dark:hover:shadow-indigo-400/25 hover:-translate-y-0.5 ring-1 ring-white/20 dark:ring-white/10"
                  >
                    Process
                  </button>
                  <button
                    @click="clearText"
                    :disabled="!hasText"
                    class="px-2 py-1 text-xs font-medium text-slate-600 dark:text-slate-300 hover:text-indigo-500 dark:hover:text-indigo-400 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                  >
                    Clear
                  </button>
                </div>
              </div>
              <div class="absolute inset-0 bg-white/60 dark:bg-gray-900/60 backdrop-blur-sm flex items-center justify-center rounded-2xl" v-if="isProcessing">
                <div class="flex items-center space-x-3">
                  <div class="relative">
                    <div class="w-12 h-12 border-2 border-indigo-500/20 dark:border-indigo-400/20 rounded-full animate-ping absolute inset-0"></div>
                    <div class="w-12 h-12 border-2 border-indigo-500 dark:border-indigo-400 border-t-transparent rounded-full animate-spin"></div>
                  </div>
                  <span class="text-sm font-medium text-slate-600 dark:text-slate-400">Processing...</span>
                </div>
              </div>
              <EditorContent :editor="editor" />
            </div>
          </div>

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
import { ref, watch, onMounted, computed } from 'vue'
import { useDark } from '@vueuse/core'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import DedupSettings from './components/molecules/DedupSettings.vue'
import DuplicateResults from './components/molecules/DuplicateResults.vue'
import { invoke } from '@tauri-apps/api/core'

const isDark = useDark()

interface DedupStrategy {
  similarity_threshold: number
  case_sensitive: boolean
  ignore_whitespace: boolean
  ignore_punctuation: boolean
  [key: string]: any
}

const strategy = ref<DedupStrategy>({
  similarity_threshold: 0.7,
  case_sensitive: false,
  ignore_whitespace: true,
  ignore_punctuation: true
})

const text = ref('')
const results = ref<any[]>([])
const isProcessing = ref(false)

const hasText = computed(() => editor.value?.getText().trim().length > 0)

const editor = useEditor({
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Paste your text here to find duplicates...',
    }),
  ],
  editorProps: {
    attributes: {
      class: 'w-full h-80 pt-12 px-5 pb-5 bg-white dark:bg-gray-900 rounded-2xl ring-2 ring-dashed ring-slate-300/70 dark:ring-gray-700/70 hover:ring-indigo-500/50 dark:hover:ring-indigo-400/50 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 focus:outline-none resize-none text-slate-700 dark:text-gray-100 placeholder-slate-400 dark:placeholder-gray-500 transition-all duration-200 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:repeating-linear-gradient(135deg,rgba(99,102,241,0.012)_0px,rgba(99,102,241,0.012)_2px,transparent_2px,transparent_4px)] dark:[background-image:repeating-linear-gradient(135deg,rgba(129,140,248,0.012)_0px,rgba(129,140,248,0.012)_2px,transparent_2px,transparent_4px)] [background-size:4px_4px] prose prose-sm max-w-none prose-slate dark:prose-invert',
    },
  },
  autofocus: 'end',
})

// Sync editor content with text ref
watch(editor, () => {
  if (editor.value) {
    text.value = editor.value.getText()
  }
}, { deep: true })

onMounted(() => {
  // Focus editor when component mounts
  setTimeout(() => {
    editor.value?.commands.focus('end')
  }, 100)
})

async function findDuplicates() {
  if (!hasText.value) return
  
  isProcessing.value = true
  try {
    const duplicates = await invoke('find_duplicates', {
      text: editor.value?.getText() || '',
      strategy: {
        ...strategy.value,
        similarity_threshold: Number(strategy.value.similarity_threshold)
      }
    })
    results.value = duplicates as any[]
  } catch (error) {
    console.error('Error finding duplicates:', error)
  } finally {
    isProcessing.value = false
  }
}

function addTestText() {
  const testText = `Here is some sample text with duplicates:
This is a test sentence.
This is another sentence.
This is a test sentence.
Here is some different text.
This is another sentence.
Here is some different text.`
  
  editor.value?.commands.setContent(testText)
}

function clearText() {
  if (!hasText.value) return
  editor.value?.commands.clearContent()
}

// Watch for changes in text or strategy to auto-update results
watch([text, strategy], async () => {
  if (text.value.trim()) {
    await findDuplicates()
  }
}, { deep: true })
</script>