<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'

enum SplitStrategy {
  Characters = 'Characters',
  Words = 'Words',
  Sentences = 'Sentences',
  Paragraphs = 'Paragraphs',
  WholeText = 'WholeText'
}

enum ComparisonScope {
  WithinUnit = 'WithinUnit',
  AcrossUnits = 'AcrossUnits',
  Both = 'Both'
}

interface DedupStrategy {
  case_sensitive: boolean
  ignore_whitespace: boolean
  ignore_punctuation: boolean
  normalize_unicode: boolean
  split_strategy: SplitStrategy
  comparison_scope: ComparisonScope
  min_length: number
  similarity_threshold: number
}

const duplicates = ref<string[][]>([])
const strategy = ref<DedupStrategy>({
  case_sensitive: false,
  ignore_whitespace: true,
  ignore_punctuation: false,
  normalize_unicode: false,
  split_strategy: SplitStrategy.Sentences,
  comparison_scope: ComparisonScope.Both,
  min_length: 0,
  similarity_threshold: 0.8
})

const editor = useEditor({
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Enter your text here...'
    })
  ],
  content: '',
  editorProps: {
    attributes: {
      class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl mx-auto focus:outline-none'
    }
  }
})

function handleStrategyChange<K extends keyof DedupStrategy>(
  key: K,
  value: DedupStrategy[K]
) {
  console.log(`Strategy change: ${key} = ${value}`)
  strategy.value[key] = value
  // Update strategy in backend
  invoke('update_strategy', {
    strategy: strategy.value
  }).catch(error => {
    console.error('Error updating strategy:', error)
  })
}

// Initialize strategy from backend on mount
onMounted(async () => {
  try {
    strategy.value = await invoke<DedupStrategy>('get_strategy')
  } catch (error) {
    console.error('Error getting strategy:', error)
  }
})

async function handleSubmit() {
  if (!editor.value?.getText()) {
    console.log('No text to process')
    return
  }
  
  const text = editor.value.getText()
  console.log('Processing text:', text)
  
  try {
    console.log('Updating strategy:', strategy.value)
    await invoke('update_strategy', {
      strategy: strategy.value
    })
    
    console.log('Processing text with backend')
    await invoke('process_text', {
      content: text
    })
    
    console.log('Getting duplicates')
    const result = await invoke<string[][]>('get_duplicates')
    console.log('Got duplicates:', result)
    duplicates.value = result
  } catch (error) {
    console.error('Error finding duplicates:', error)
  }
}

function handleClear() {
  editor.value?.commands.clearContent()
  duplicates.value = []
}

function addTestDuplicate() {
  const testText = `This is a test sentence.
Another unique sentence here.
This is a test sentence.
Some more text.
Another unique sentence here.`
  
  editor.value?.commands.setContent(testText)
}
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm">
      <div class="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
        <h1 class="text-lg font-semibold text-gray-900">Text Deduplication</h1>
      </div>
    </header>

    <main class="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
      <div class="bg-white rounded-lg shadow">
        <!-- Settings Panel -->
        <div class="p-6 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900 mb-4">Settings</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <!-- Split Strategy -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Split Strategy
              </label>
              <select
                v-model="strategy.split_strategy"
                @change="(e: Event) => {
                  const target = e.target as HTMLSelectElement | null;
                  if (target) {
                    handleStrategyChange('split_strategy', target.value as SplitStrategy);
                  }
                }"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
              >
                <option v-for="opt in Object.values(SplitStrategy)" :key="opt" :value="opt">
                  {{ opt }}
                </option>
              </select>
            </div>

            <!-- Comparison Scope -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Comparison Scope
              </label>
              <select
                v-model="strategy.comparison_scope"
                @change="(e: Event) => {
                  const target = e.target as HTMLSelectElement | null;
                  if (target) {
                    handleStrategyChange('comparison_scope', target.value as ComparisonScope);
                  }
                }"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
              >
                <option v-for="opt in Object.values(ComparisonScope)" :key="opt" :value="opt">
                  {{ opt }}
                </option>
              </select>
            </div>

            <!-- Similarity Threshold -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Similarity Threshold
              </label>
              <input
                type="range"
                v-model.number="strategy.similarity_threshold"
                min="0"
                max="1"
                step="0.1"
                @change="(e: Event) => {
                  const target = e.target as HTMLInputElement;
                  handleStrategyChange('similarity_threshold', Number(target.value));
                }"
                class="mt-1 block w-full"
              />
              <div class="text-sm text-gray-500 mt-1">
                {{ Math.round((strategy.similarity_threshold || 0) * 100) }}%
              </div>
            </div>

            <!-- Case Sensitive -->
            <div>
              <label class="flex items-center">
                <input
                  type="checkbox"
                  v-model="strategy.case_sensitive"
                  @change="(e: Event) => {
                    const target = e.target as HTMLInputElement;
                    handleStrategyChange('case_sensitive', target.checked);
                  }"
                  class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                />
                <span class="ml-2 text-sm text-gray-700">Case Sensitive</span>
              </label>
            </div>

            <!-- Ignore Whitespace -->
            <div>
              <label class="flex items-center">
                <input
                  type="checkbox"
                  v-model="strategy.ignore_whitespace"
                  @change="(e: Event) => {
                    const target = e.target as HTMLInputElement;
                    handleStrategyChange('ignore_whitespace', target.checked);
                  }"
                  class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                />
                <span class="ml-2 text-sm text-gray-700">Ignore Whitespace</span>
              </label>
            </div>

            <!-- Ignore Punctuation -->
            <div>
              <label class="flex items-center">
                <input
                  type="checkbox"
                  v-model="strategy.ignore_punctuation"
                  @change="(e: Event) => {
                    const target = e.target as HTMLInputElement;
                    handleStrategyChange('ignore_punctuation', target.checked);
                  }"
                  class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                />
                <span class="ml-2 text-sm text-gray-700">Ignore Punctuation</span>
              </label>
            </div>

            <!-- Normalize Unicode -->
            <div>
              <label class="flex items-center">
                <input
                  type="checkbox"
                  v-model="strategy.normalize_unicode"
                  @change="(e: Event) => {
                    const target = e.target as HTMLInputElement;
                    handleStrategyChange('normalize_unicode', target.checked);
                  }"
                  class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
                />
                <span class="ml-2 text-sm text-gray-700">Normalize Unicode</span>
              </label>
            </div>

            <!-- Min Length -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Min Length
              </label>
              <input
                type="number"
                v-model.number="strategy.min_length"
                @change="(e: Event) => {
                  const target = e.target as HTMLInputElement;
                  handleStrategyChange('min_length', Number(target.value));
                }"
                class="mt-1 block w-full"
              />
            </div>
          </div>
        </div>

        <!-- Editor Section -->
        <div class="p-6">
          <div class="mb-4 flex justify-between">
            <div class="space-x-2">
              <button
                @click="handleSubmit"
                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                Find Duplicates
              </button>
              <button
                @click="handleClear"
                class="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                Clear
              </button>
            </div>
            <button
              @click="addTestDuplicate"
              class="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              Add Test Text
            </button>
          </div>

          <!-- TipTap Editor -->
          <div class="prose prose-sm sm:prose lg:prose-lg xl:prose-2xl mx-auto">
            <editor-content :editor="editor" class="min-h-[200px] border rounded-lg p-4" />
          </div>

          <!-- Results Section -->
          <div v-if="duplicates.length > 0" class="mt-6 p-4 bg-gray-50 rounded-lg">
            <h3 class="text-lg font-medium text-gray-900 mb-4">Duplicate Groups Found</h3>
            <div class="space-y-4">
              <div v-for="(group, groupIndex) in duplicates" :key="groupIndex" 
                   class="p-4 bg-white shadow rounded-lg">
                <div class="text-sm font-medium text-gray-500 mb-2">
                  Group {{ groupIndex + 1 }} ({{ group.length }} occurrences)
                </div>
                <div class="space-y-2">
                  <div v-for="(text, index) in group" :key="index" 
                       class="p-2 bg-gray-50 rounded text-gray-700">
                    {{ text }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>

</style>