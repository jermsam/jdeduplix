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

const currentTab = ref('text')
const showSettings = ref(false)
const duplicates = ref<string[][]>([])

const tabs = [
  { id: 'text', name: 'Text' },
  { id: 'json', name: 'JSON' },
  { id: 'image', name: 'Images' },
  { id: 'binary', name: 'Binary' }
]

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
      class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl focus:outline-none'
    }
  }
})

function handleStrategyChange<K extends keyof DedupStrategy>(
  key: K,
  value: DedupStrategy[K]
) {
  console.log(`Strategy change: ${key} = ${value}`)
  strategy.value[key] = value
  invoke('update_strategy', {
    strategy: strategy.value
  }).catch(error => {
    console.error('Error updating strategy:', error)
  })
}

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
    <!-- Header with Navigation -->
    <header class="bg-white shadow-sm sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center">
          <h1 class="text-xl font-semibold text-gray-900">JDeduplix</h1>
          <nav class="flex space-x-4">
            <button 
              v-for="tab in tabs" 
              :key="tab.id"
              @click="currentTab = tab.id"
              :class="[
                'px-3 py-2 rounded-md text-sm font-medium',
                currentTab === tab.id
                  ? 'bg-indigo-100 text-indigo-700'
                  : 'text-gray-500 hover:text-gray-700'
              ]"
            >
              {{ tab.name }}
            </button>
          </nav>
        </div>
      </div>
    </header>

    <main class="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
      <!-- Text Deduplication Tab -->
      <div v-if="currentTab === 'text'" class="space-y-6">
        <div class="bg-white rounded-lg shadow">
          <!-- Main Content Area -->
          <div class="p-6">
            <div class="mb-6">
              <h2 class="text-lg font-medium text-gray-900 mb-2">Input Text</h2>
              <div class="border rounded-lg">
                <editor-content :editor="editor" />
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex justify-between items-center mb-6">
              <div class="flex space-x-4">
                <button
                  @click="handleSubmit"
                  class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                  Find Duplicates
                </button>
                <button
                  @click="handleClear"
                  class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                  Clear
                </button>
                <button
                  @click="addTestDuplicate"
                  class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                  Add Test Text
                </button>
              </div>
              <button
                @click="showSettings = !showSettings"
                class="inline-flex items-center px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                <svg class="h-5 w-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                </svg>
                Settings
              </button>
            </div>

            <!-- Settings Panel (Collapsible) -->
            <div v-if="showSettings" class="bg-gray-50 rounded-lg p-6 mb-6">
              <h3 class="text-lg font-medium text-gray-900 mb-4">Deduplication Settings</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <!-- Basic Settings -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-700">Basic Options</h4>
                  <div>
                    <label class="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        v-model="strategy.case_sensitive"
                        @change="(e) => handleStrategyChange('case_sensitive', e.target.checked)"
                        class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <span class="text-sm text-gray-700">Case Sensitive</span>
                    </label>
                  </div>
                  <div>
                    <label class="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        v-model="strategy.ignore_whitespace"
                        @change="(e) => handleStrategyChange('ignore_whitespace', e.target.checked)"
                        class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <span class="text-sm text-gray-700">Ignore Whitespace</span>
                    </label>
                  </div>
                  <div>
                    <label class="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        v-model="strategy.ignore_punctuation"
                        @change="(e) => handleStrategyChange('ignore_punctuation', e.target.checked)"
                        class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <span class="text-sm text-gray-700">Ignore Punctuation</span>
                    </label>
                  </div>
                  <div>
                    <label class="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        v-model="strategy.normalize_unicode"
                        @change="(e) => handleStrategyChange('normalize_unicode', e.target.checked)"
                        class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <span class="text-sm text-gray-700">Normalize Unicode</span>
                    </label>
                  </div>
                </div>

                <!-- Advanced Settings -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-700">Advanced Options</h4>
                  <div>
                    <label class="block text-sm text-gray-700 mb-1">Split Strategy</label>
                    <select
                      v-model="strategy.split_strategy"
                      @change="(e) => handleStrategyChange('split_strategy', e.target.value)"
                      class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    >
                      <option v-for="opt in Object.values(SplitStrategy)" :key="opt" :value="opt">
                        {{ opt }}
                      </option>
                    </select>
                  </div>
                  <div>
                    <label class="block text-sm text-gray-700 mb-1">Comparison Scope</label>
                    <select
                      v-model="strategy.comparison_scope"
                      @change="(e) => handleStrategyChange('comparison_scope', e.target.value)"
                      class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    >
                      <option v-for="opt in Object.values(ComparisonScope)" :key="opt" :value="opt">
                        {{ opt }}
                      </option>
                    </select>
                  </div>
                </div>

                <!-- Thresholds -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-700">Thresholds</h4>
                  <div>
                    <label class="block text-sm text-gray-700 mb-1">
                      Similarity Threshold: {{ strategy.similarity_threshold }}
                    </label>
                    <input
                      type="range"
                      v-model.number="strategy.similarity_threshold"
                      min="0"
                      max="1"
                      step="0.1"
                      @change="(e) => handleStrategyChange('similarity_threshold', Number(e.target.value))"
                      class="mt-1 block w-full"
                    />
                  </div>
                  <div>
                    <label class="block text-sm text-gray-700 mb-1">
                      Minimum Length: {{ strategy.min_length }}
                    </label>
                    <input
                      type="number"
                      v-model.number="strategy.min_length"
                      min="0"
                      @change="(e) => handleStrategyChange('min_length', Number(e.target.value))"
                      class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    />
                  </div>
                </div>
              </div>
            </div>

            <!-- Results -->
            <div v-if="duplicates.length > 0" class="bg-white rounded-lg">
              <h3 class="text-lg font-medium text-gray-900 mb-4">Duplicate Groups</h3>
              <div class="space-y-4">
                <div 
                  v-for="(group, groupIndex) in duplicates" 
                  :key="groupIndex"
                  class="bg-gray-50 rounded-lg p-4"
                >
                  <div class="text-sm font-medium text-gray-500 mb-2">
                    Group {{ groupIndex + 1 }} ({{ group.length }} occurrences)
                  </div>
                  <div class="space-y-2">
                    <div 
                      v-for="(text, index) in group" 
                      :key="index"
                      class="bg-white p-3 rounded border border-gray-200"
                    >
                      {{ text }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Placeholder for future tabs -->
      <div v-else-if="currentTab === 'json'" class="bg-white rounded-lg shadow p-6">
        <div class="text-center text-gray-500">
          JSON Deduplication coming soon...
        </div>
      </div>
      <div v-else-if="currentTab === 'image'" class="bg-white rounded-lg shadow p-6">
        <div class="text-center text-gray-500">
          Image Deduplication coming soon...
        </div>
      </div>
      <div v-else-if="currentTab === 'binary'" class="bg-white rounded-lg shadow p-6">
        <div class="text-center text-gray-500">
          Binary Deduplication coming soon...
        </div>
      </div>
    </main>
  </div>
</template>

<style>
.ProseMirror {
  @apply min-h-[200px] p-4;
}

.ProseMirror:focus {
  @apply outline-none;
}

.ProseMirror p.is-editor-empty:first-child::before {
  content: attr(data-placeholder);
  @apply text-gray-400 float-left h-0 pointer-events-none;
}
</style>