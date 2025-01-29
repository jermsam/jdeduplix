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
  <div class="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50">
    <!-- Premium Header with Glass Effect -->
    <header class="sticky top-0 z-50 backdrop-blur-md bg-white/70 border-b border-white/20">
      <div class="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center">
          <h1 class="text-2xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
            JDeduplix
          </h1>
          <nav class="flex space-x-2">
            <button 
              v-for="tab in tabs" 
              :key="tab.id"
              @click="currentTab = tab.id"
              :class="[
                'px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ease-in-out',
                currentTab === tab.id
                  ? 'bg-gradient-to-r from-indigo-500 to-purple-500 text-white shadow-lg shadow-indigo-500/20'
                  : 'text-gray-600 hover:text-gray-900 hover:bg-white/50'
              ]"
            >
              {{ tab.name }}
            </button>
          </nav>
        </div>
      </div>
    </header>

    <main class="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
      <!-- Text Deduplication Tab -->
      <div v-if="currentTab === 'text'" class="space-y-6">
        <div class="backdrop-blur-xl bg-white/80 rounded-2xl shadow-xl border border-white/20">
          <!-- Main Content Area -->
          <div class="p-8">
            <div class="mb-8">
              <h2 class="text-xl font-semibold text-gray-900 mb-4">Input Text</h2>
              <div class="backdrop-blur-sm bg-white/50 border border-white/20 rounded-xl overflow-hidden transition-all duration-200 hover:shadow-lg">
                <editor-content :editor="editor" />
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex justify-between items-center mb-8">
              <div class="flex space-x-4">
                <button
                  @click="handleSubmit"
                  class="px-6 py-3 rounded-xl font-medium text-white bg-gradient-to-r from-indigo-500 to-purple-500 hover:from-indigo-600 hover:to-purple-600 shadow-lg shadow-indigo-500/20 transition-all duration-200 hover:shadow-xl hover:-translate-y-0.5"
                >
                  Find Duplicates
                </button>
                <button
                  @click="handleClear"
                  class="px-6 py-3 rounded-xl font-medium text-gray-700 bg-white/80 hover:bg-white/90 border border-white/20 shadow-lg transition-all duration-200 hover:shadow-xl hover:-translate-y-0.5"
                >
                  Clear
                </button>
                <button
                  @click="addTestDuplicate"
                  class="px-6 py-3 rounded-xl font-medium text-gray-700 bg-white/80 hover:bg-white/90 border border-white/20 shadow-lg transition-all duration-200 hover:shadow-xl hover:-translate-y-0.5"
                >
                  Add Test Text
                </button>
              </div>
              <button
                @click="showSettings = !showSettings"
                class="group px-4 py-3 rounded-xl font-medium text-gray-700 bg-white/80 hover:bg-white/90 border border-white/20 shadow-lg transition-all duration-200 hover:shadow-xl hover:-translate-y-0.5 flex items-center space-x-2"
              >
                <svg class="h-5 w-5 transition-transform duration-200 group-hover:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                </svg>
                <span>Settings</span>
              </button>
            </div>

            <!-- Settings Panel (Collapsible) -->
            <div v-if="showSettings" 
                 class="backdrop-blur-xl bg-white/60 rounded-2xl p-8 mb-8 border border-white/20 shadow-lg transition-all duration-300"
                 v-motion
                 :initial="{ opacity: 0, y: 20 }"
                 :enter="{ opacity: 1, y: 0 }">
              <h3 class="text-xl font-semibold text-gray-900 mb-6">Deduplication Settings</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                <!-- Basic Settings -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-900 mb-4">Basic Options</h4>
                  <div class="space-y-3">
                    <label v-for="(option, key) in {
                      case_sensitive: 'Case Sensitive',
                      ignore_whitespace: 'Ignore Whitespace',
                      ignore_punctuation: 'Ignore Punctuation',
                      normalize_unicode: 'Normalize Unicode'
                    }" :key="key" 
                    class="flex items-center space-x-3 p-3 rounded-xl bg-white/50 hover:bg-white/70 transition-colors duration-200">
                      <input
                        type="checkbox"
                        v-model="strategy[key]"
                        @change="(e) => handleStrategyChange(key, e.target.checked)"
                        class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 transition-all duration-200"
                      />
                      <span class="text-sm text-gray-700">{{ option }}</span>
                    </label>
                  </div>
                </div>

                <!-- Advanced Settings -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-900 mb-4">Advanced Options</h4>
                  <div class="space-y-4">
                    <div class="space-y-2">
                      <label class="block text-sm text-gray-700">Split Strategy</label>
                      <select
                        v-model="strategy.split_strategy"
                        @change="(e) => handleStrategyChange('split_strategy', e.target.value)"
                        class="block w-full rounded-xl border-gray-300 bg-white/50 hover:bg-white/70 transition-colors duration-200 focus:border-indigo-500 focus:ring-indigo-500"
                      >
                        <option v-for="opt in Object.values(SplitStrategy)" :key="opt" :value="opt">
                          {{ opt }}
                        </option>
                      </select>
                    </div>
                    <div class="space-y-2">
                      <label class="block text-sm text-gray-700">Comparison Scope</label>
                      <select
                        v-model="strategy.comparison_scope"
                        @change="(e) => handleStrategyChange('comparison_scope', e.target.value)"
                        class="block w-full rounded-xl border-gray-300 bg-white/50 hover:bg-white/70 transition-colors duration-200 focus:border-indigo-500 focus:ring-indigo-500"
                      >
                        <option v-for="opt in Object.values(ComparisonScope)" :key="opt" :value="opt">
                          {{ opt }}
                        </option>
                      </select>
                    </div>
                  </div>
                </div>

                <!-- Thresholds -->
                <div class="space-y-4">
                  <h4 class="font-medium text-gray-900 mb-4">Thresholds</h4>
                  <div class="space-y-4">
                    <div class="space-y-2">
                      <label class="flex justify-between text-sm text-gray-700">
                        <span>Similarity Threshold</span>
                        <span class="text-indigo-600">{{ Math.round(strategy.similarity_threshold * 100) }}%</span>
                      </label>
                      <input
                        type="range"
                        v-model.number="strategy.similarity_threshold"
                        min="0"
                        max="1"
                        step="0.1"
                        @change="(e) => handleStrategyChange('similarity_threshold', Number(e.target.value))"
                        class="w-full accent-indigo-600"
                      />
                    </div>
                    <div class="space-y-2">
                      <label class="block text-sm text-gray-700">Minimum Length</label>
                      <input
                        type="number"
                        v-model.number="strategy.min_length"
                        min="0"
                        @change="(e) => handleStrategyChange('min_length', Number(e.target.value))"
                        class="block w-full rounded-xl border-gray-300 bg-white/50 hover:bg-white/70 transition-colors duration-200 focus:border-indigo-500 focus:ring-indigo-500"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Results -->
            <div v-if="duplicates.length > 0" 
                 class="space-y-6"
                 v-motion
                 :initial="{ opacity: 0, y: 20 }"
                 :enter="{ opacity: 1, y: 0 }">
              <h3 class="text-xl font-semibold text-gray-900">Duplicate Groups</h3>
              <div class="space-y-6">
                <div 
                  v-for="(group, groupIndex) in duplicates" 
                  :key="groupIndex"
                  class="backdrop-blur-xl bg-white/60 rounded-2xl p-6 border border-white/20 shadow-lg"
                >
                  <div class="text-sm font-medium text-indigo-600 mb-4 flex items-center">
                    <span class="bg-indigo-100 text-indigo-700 px-3 py-1 rounded-lg">
                      Group {{ groupIndex + 1 }}
                    </span>
                    <span class="ml-3 text-gray-500">
                      {{ group.length }} occurrences
                    </span>
                  </div>
                  <div class="space-y-3">
                    <div 
                      v-for="(text, index) in group" 
                      :key="index"
                      class="bg-white/70 p-4 rounded-xl border border-white/20 shadow-sm hover:shadow-md transition-all duration-200"
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
      <div v-else class="backdrop-blur-xl bg-white/80 rounded-2xl shadow-xl border border-white/20 p-8">
        <div class="text-center">
          <div class="text-6xl mb-4">🚧</div>
          <h3 class="text-xl font-semibold text-gray-900 mb-2">Coming Soon</h3>
          <p class="text-gray-500">
            {{ currentTab.charAt(0).toUpperCase() + currentTab.slice(1) }} deduplication is under development
          </p>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
.ProseMirror {
  @apply min-h-[200px] p-6 text-gray-700;
}

.ProseMirror:focus {
  @apply outline-none;
}

.ProseMirror p.is-editor-empty:first-child::before {
  content: attr(data-placeholder);
  @apply text-gray-400 float-left h-0 pointer-events-none;
}

/* Smooth scrolling */
html {
  scroll-behavior: smooth;
}

/* Custom scrollbar */
::-webkit-scrollbar {
  @apply w-2;
}

::-webkit-scrollbar-track {
  @apply bg-transparent;
}

::-webkit-scrollbar-thumb {
  @apply bg-gray-300 rounded-full hover:bg-gray-400 transition-colors duration-200;
}

/* Animations */
.fade-enter-active,
.fade-leave-active {
  @apply transition-opacity duration-300;
}

.fade-enter-from,
.fade-leave-to {
  @apply opacity-0;
}

/* Range input styling */
input[type="range"] {
  @apply h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer;
}

input[type="range"]::-webkit-slider-thumb {
  @apply appearance-none w-4 h-4 bg-indigo-600 rounded-full shadow cursor-pointer transition-all duration-200 hover:scale-110;
}
</style>