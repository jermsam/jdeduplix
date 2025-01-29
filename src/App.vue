<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'

// Editor extensions
import { Placeholder } from '@tiptap/extension-placeholder'

// Component imports
import Container from './components/atoms/Container.vue'
import Text from './components/atoms/Text.vue'
import DedupSettings from './components/molecules/DedupSettings.vue'
import Button from './components/atoms/Button.vue'

// Types
import { DedupStrategy, DuplicateGroup, DEFAULT_STRATEGY, SplitStrategy, ComparisonScope } from './types/dedup'

// Theme handling
const isDark = ref(true) // Default to dark theme
const theme = ref(isDark.value ? 'dark' : 'light')

watch(isDark, (newValue) => {
  theme.value = newValue ? 'dark' : 'light'
  document.documentElement.classList.toggle('dark', newValue)
})

onMounted(() => {
  // Initialize theme
  document.documentElement.classList.toggle('dark', isDark.value)
})

// Tab handling
const currentTab = ref('text')
const isSettingsExpanded = ref(false)
const duplicates = ref<DuplicateGroup[]>([])
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

const tabs = [
  { id: 'text', name: 'Text' },
  { id: 'json', name: 'JSON' },
  { id: 'image', name: 'Images' },
  { id: 'binary', name: 'Binary' }
]

const comparisonScopes = [
  { id: 'WITHINUNIT', name: 'Within Unit' },
  { id: 'ACROSSUNITS', name: 'Across Units' },
  { id: 'BOTH', name: 'Both' }
]

const splitStrategies = [
  { id: 'CHARACTERS', name: 'Characters' },
  { id: 'WORDS', name: 'Words' },
  { id: 'SENTENCES', name: 'Sentences' },
  { id: 'PARAGRAPHS', name: 'Paragraphs' },
  { id: 'WHOLETEXT', name: 'Whole Text' }
]

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
  console.log('App: Strategy change:', key, value)
  strategy.value = {
    ...strategy.value,
    [key]: value
  }
  
  // Save strategy changes to backend
  invoke('set_strategy', { strategy: strategy.value })
    .catch(error => {
      console.error('Failed to save strategy:', error)
    })
}

async function handleSubmit() {
  if (!editor.value?.getText()) {
    console.log('No text to process')
    return
  }
  
  const text = editor.value.getText()
  console.log('Processing text:', text)
  
  try {
    await invoke('update_strategy', {
      strategy: strategy.value
    })
    
    console.log('Processing text with backend')
    const result = await invoke<DuplicateGroup[]>('process_text', {
      text: text
    })
    
    console.log('Got duplicates:', JSON.stringify(result, null, 2))
    duplicates.value = result || []
  } catch (error) {
    console.error('Error finding duplicates:', error)
    duplicates.value = []
  }
}

onMounted(async () => {
  try {
    const savedStrategy = await invoke<DedupStrategy>('get_strategy')
    if (savedStrategy) {
      strategy.value = savedStrategy
    }
  } catch (error) {
    console.error('Error getting strategy:', error)
  }
})

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
  <div class="min-h-screen bg-[#0F1115] text-gray-100">
    <main class="p-6">
      <Container maxWidth="2xl">
        <div class="space-y-6">
          <!-- Action Buttons -->
          <div class="flex items-center gap-3">
            <Button variant="primary" @click="handleSubmit" class="bg-indigo-600 hover:bg-indigo-500">
              Find Duplicates
            </Button>
            <Button variant="ghost" @click="handleClear" class="text-gray-300 hover:bg-gray-800">
              Clear
            </Button>
            <Button variant="ghost" @click="addTestDuplicate" class="text-gray-300 hover:bg-gray-800">
              Add Test Text
            </Button>
          </div>

          <!-- Editor and Settings -->
          <div class="space-y-6">
            <!-- Editor -->
            <div class="rounded-lg overflow-hidden bg-[#1A1D23]">
              <EditorContent :editor="editor" class="p-4" />
            </div>

            <!-- Settings -->
            <DedupSettings
              v-model:strategy="strategy.value"
              :is-dark="true"
            />
          </div>

          <!-- Results -->
          <div v-if="duplicates.length > 0" class="space-y-4">
            <div
              v-for="(group, idx) in duplicates"
              :key="idx"
              class="rounded-lg overflow-hidden bg-[#1A1D23]"
            >
              <div class="p-3 border-b border-gray-700 flex items-center justify-between">
                <Text size="sm" class="text-gray-400">
                  Similarity: {{ Math.round(group.similarity * 100) }}%
                </Text>
              </div>
              <div class="p-4 space-y-3">
                <div class="rounded p-3 bg-[#1E2128]">
                  {{ group.original }}
                </div>
                <div
                  v-for="(text, textIdx) in group.duplicates"
                  :key="textIdx"
                  class="rounded p-3 bg-[#1E2128]"
                >
                  {{ text }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </Container>
    </main>
  </div>
</template>

<style>
.ProseMirror {
  @apply min-h-[200px] p-4 outline-none;
}

.ProseMirror p {
  @apply mb-4 last:mb-0;
}

.dark .ProseMirror {
  @apply text-gray-100;
}
</style>