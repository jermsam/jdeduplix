<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Placeholder } from '@tiptap/extension-placeholder'

// Component imports
import Container from './components/atoms/Container.vue'
import Text from './components/atoms/Text.vue'
import DedupSettings from './components/molecules/DedupSettings.vue'
import DuplicateResults from './components/molecules/DuplicateResults.vue'
import Button from './components/atoms/Button.vue'

// Composables
import { useDeduplication } from './composables/useDeduplication'

// Theme handling
const isDark = ref(true) // Default to dark theme
const theme = ref(isDark.value ? 'dark' : 'light')

// Initialize deduplication
const { strategy, duplicates, findDuplicates, clearDuplicates, loadSavedStrategy } = useDeduplication()

// Handle deletion of duplicates
const handleDelete = (index: number) => {
  duplicates.value = duplicates.value.filter((_, i) => i !== index)
}

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
  if (editor.value) {
    editor.value.commands.setContent('')
  }
  clearDuplicates()
}

// Add test duplicate text
const addTestDuplicate = () => {
  if (editor.value) {
    const text = `Project Meeting Notes

The quarterly review meeting is scheduled for 3PM on Thursday. All team leads must attend and prepare their status reports. Please ensure your reports are submitted by Wednesday COB.

The Q4 review meeting will be held at 3pm on Thursday. Team leads are required to attend with their status reports. Submit all reports before close of business on Wednesday.

Key project deliverables include implementing the new authentication system and updating the user interface. The dev team estimates 4-6 weeks for the authentication system implementation. We need to carefully test all security aspects before deployment.

Among our main deliverables, we'll be working on a new auth system and UI updates. The development team thinks the authentication implementation will take 4 to 6 weeks. Security testing must be thorough before we can deploy.

IMPORTANT: Please review the attached security guidelines. These guidelines MUST be followed for all code changes.

Important - All developers should read the security guidelines attached. Following these guidelines is mandatory for any code modifications.

Contact Sarah@company.com for technical questions. You can reach out to sarah@company.com if you need technical assistance. For general inquiries, email support@company.com.`;

    editor.value.commands.setContent(text);
  }
}
</script>

<template>
  <div class="min-h-screen bg-gradient-to-b from-[#141517] to-[#1A1D23] text-gray-100">
    <nav class="border-b border-gray-800">
      <div class="container mx-auto px-4 py-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-8">
            <div>
              <h1 class="text-lg font-medium bg-gradient-to-r from-indigo-400 to-indigo-600 bg-clip-text text-transparent">Jdeduplix</h1>
              <p class="text-xs text-gray-400 mt-0.5">Smart Deduplication System</p>
            </div>
            <div class="hidden sm:flex items-center gap-1">
              <button class="px-3 py-1.5 text-xs font-medium rounded-lg bg-indigo-500 bg-opacity-10 text-indigo-400">Text</button>
              <button class="px-3 py-1.5 text-xs font-medium rounded-lg text-gray-400 hover:bg-[#1E2128] transition-colors" disabled>JSON</button>
              <button class="px-3 py-1.5 text-xs font-medium rounded-lg text-gray-400 hover:bg-[#1E2128] transition-colors" disabled>Images</button>
              <button class="px-3 py-1.5 text-xs font-medium rounded-lg text-gray-400 hover:bg-[#1E2128] transition-colors" disabled>Binary</button>
            </div>
          </div>
          <div class="flex items-center gap-3">
            <button
              @click="handleSubmit"
              :disabled="!editor?.getText()"
              class="px-4 py-1.5 text-xs font-medium rounded-lg bg-gradient-to-r from-indigo-500 to-indigo-600 hover:from-indigo-600 hover:to-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg shadow-indigo-500/20 ring-1 ring-indigo-500/50 flex items-center gap-2"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              Find Duplicates
            </button>
            <a href="https://github.com/jermsam/jdeduplix" target="_blank" class="p-1.5 text-gray-400 hover:text-gray-300 transition-colors rounded-lg hover:bg-[#1E2128]">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </nav>
    <div class="container mx-auto px-4 py-6 max-w-7xl">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Editor Section -->
        <div class="lg:col-span-2 space-y-4">
          <div class="bg-[#1A1D23] rounded-lg overflow-hidden ring-1 ring-gray-800 shadow-xl">
            <div class="flex items-center justify-between p-2 bg-[#1E2128] border-b border-gray-800">
              <div class="flex items-center gap-2">
                <div class="flex items-center gap-1.5">
                  <div class="w-2.5 h-2.5 rounded-full bg-red-500"></div>
                  <div class="w-2.5 h-2.5 rounded-full bg-yellow-500"></div>
                  <div class="w-2.5 h-2.5 rounded-full bg-green-500"></div>
                </div>
                <div class="text-xs font-medium text-gray-300 ml-2">Editor</div>
              </div>
              <div class="flex items-center gap-2">
                <button
                  @click="handleClear"
                  class="px-2 py-1 text-xs text-gray-400 hover:text-gray-300 transition-colors group flex items-center gap-1"
                >
                  <svg class="w-3.5 h-3.5 text-gray-500 group-hover:text-gray-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                  Clear
                </button>
                <button
                  @click="addTestDuplicate"
                  class="px-2 py-1 text-xs text-gray-400 hover:text-gray-300 transition-colors group flex items-center gap-1"
                >
                  <svg class="w-3.5 h-3.5 text-gray-500 group-hover:text-gray-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  Add Test Text
                </button>
              </div>
            </div>
            <div class="p-3">
              <EditorContent 
                :editor="editor" 
                class="prose prose-sm prose-invert max-w-none"
              />
            </div>
          </div>

          <!-- Results -->
          <DuplicateResults
            :duplicates="duplicates"
            @delete="handleDelete"
          />
        </div>

        <!-- Settings -->
        <div class="space-y-4 lg:h-[calc(100vh-8rem)] lg:sticky lg:top-6">
          <DedupSettings
            v-model:strategy="strategy"
            :isDark="isDark"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.ProseMirror {
  @apply min-h-[calc(100vh-20rem)] lg:min-h-[30rem] text-xs text-gray-300 font-mono;
}

.ProseMirror p.is-editor-empty:first-child::before {
  @apply text-gray-500;
  content: "Paste your text here...";
  float: left;
  pointer-events: none;
  height: 0;
}

.ProseMirror:focus {
  @apply outline-none;
}

/* Scrollbar Styles */
::-webkit-scrollbar {
  @apply w-1.5;
}

::-webkit-scrollbar-track {
  @apply bg-[#1A1D23];
}

::-webkit-scrollbar-thumb {
  @apply bg-gray-700 rounded-full hover:bg-gray-600 transition-colors;
}
</style>