<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Placeholder } from '@tiptap/extension-placeholder'

// Component imports
import Container from './components/atoms/Container.vue'
import Text from './components/atoms/Text.vue'
import DedupSettings from './components/molecules/DedupSettings.vue'
import Button from './components/atoms/Button.vue'

// Composables
import { useDeduplication } from './composables/useDeduplication'

// Theme handling
const isDark = ref(true) // Default to dark theme
const theme = ref(isDark.value ? 'dark' : 'light')

// Initialize deduplication
const { strategy, duplicates, findDuplicates, clearDuplicates, loadSavedStrategy } = useDeduplication()

// Load initial strategy
onMounted(async () => {
  // Initialize theme
  document.documentElement.classList.toggle('dark', isDark.value)
  
  // Load strategy from backend
  await loadSavedStrategy()
})

watch(isDark, (newValue) => {
  theme.value = newValue ? 'dark' : 'light'
  document.documentElement.classList.toggle('dark', newValue)
})

// Editor setup
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
      class: 'prose prose-sm prose-invert focus:outline-none max-w-none'
    }
  }
})

// Handle form submission
const handleSubmit = async () => {
  if (!editor.value?.getText()) return
  await findDuplicates(editor.value.getText())
}

// Clear results
const handleClear = () => {
  clearDuplicates()
  if (editor.value) {
    editor.value.commands.setContent('')
  }
}

// Add test duplicate text
const addTestDuplicate = () => {
  const testText = `This is a test sentence.
Another unique sentence here.
This is a test sentence.
Some more text.
Another unique sentence here.`
  
  if (editor.value) {
    editor.value.commands.setContent(testText)
  }
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
              v-model:strategy="strategy"
              :isDark="isDark"
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

<style lang="postcss">
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