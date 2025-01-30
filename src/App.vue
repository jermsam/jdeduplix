<template>
  <div class="min-h-screen theme-base" :class="isDark ? 'dark' : ''">
    <nav class="sticky top-0 z-50 backdrop-blur-md border-b theme-surface">
      <div class="container mx-auto px-4 py-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-8">
            <div>
              <h1 class="text-lg font-medium brand-gradient">Jdeduplix</h1>
              <p class="text-[12px] mt-0.5 text-theme-secondary">Smart Deduplication System</p>
            </div>
            <div class="hidden sm:flex items-center gap-1">
              <button class="px-3 py-1.5 font-medium rounded-lg transition-all duration-300" :class="{
                'bg-indigo-500/20 text-indigo-300 hover:bg-indigo-500/30': isDark,
                'bg-indigo-500 text-white shadow-lg shadow-indigo-500/25 hover:shadow-xl hover:shadow-indigo-500/30 hover:-translate-y-0.5': !isDark
              }">Text</button>
              <button class="px-3 py-1.5 font-medium rounded-lg transition-all duration-300" :class="{
                'text-[#8C8C8C] hover:text-[#CCCCCC] hover:bg-[#313131]': isDark,
                'text-[#6E6E6E] hover:text-[#1F1F1F] hover:bg-[#F3F3F3]': !isDark
              }" disabled>JSON</button>
              <button class="px-3 py-1.5 font-medium rounded-lg transition-all duration-300" :class="{
                'text-[#8C8C8C] hover:text-[#CCCCCC] hover:bg-[#313131]': isDark,
                'text-[#6E6E6E] hover:text-[#1F1F1F] hover:bg-[#F3F3F3]': !isDark
              }" disabled>Images</button>
              <button class="px-3 py-1.5 font-medium rounded-lg transition-all duration-300" :class="{
                'text-[#8C8C8C] hover:text-[#CCCCCC] hover:bg-[#313131]': isDark,
                'text-[#6E6E6E] hover:text-[#1F1F1F] hover:bg-[#F3F3F3]': !isDark
              }" disabled>Binary</button>
            </div>
          </div>
          <button 
            @click="toggleDarkMode"
            class="button-secondary"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="isDark" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
          </button>
        </div>
      </div>
    </nav>

    <main class="container mx-auto px-4 py-6">
      <div class="grid lg:grid-cols-3 gap-4">
        <!-- Editor Section -->
        <div class="lg:col-span-2 space-y-4">
          <div class="card">
            <div class="card-header">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <div class="flex items-center gap-1">
                    <div class="w-3 h-3 rounded-full bg-red-500"></div>
                    <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                    <div class="w-3 h-3 rounded-full bg-green-500"></div>
                  </div>
                  <span class="text-sm text-theme-secondary">Editor</span>
                </div>
                <div class="flex items-center gap-2">
                  <button 
                    @click="addTestText"
                    class="button-secondary"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button 
                    @click="() => {
                      const text = editor?.getText();
                      if (text) findDuplicates(text);
                    }"
                    class="button-secondary"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </button>
                  <button 
                    @click="clearDuplicates" 
                    class="button-secondary"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
            <div class="p-4">
              <EditorContent 
                :editor="editor" 
                class="max-w-none focus:outline-none"
                :class="{
                  '[&_p]:text-app-light-text-primary': isDark,
                  '[&_p]:text-app-dark-text-primary': !isDark
                }"
              />
            </div>
          </div>

          <!-- Results -->
          <DuplicateResults
            v-if="duplicates.length > 0"
            :duplicates="duplicates"
            :isDark="isDark"
            @delete="handleDelete"
            class="transition-all duration-500 ease-in-out shadow-app-dark"
          />
        </div>

        <!-- Settings -->
        <div class="space-y-4 lg:h-[calc(100vh-8rem)] lg:sticky lg:top-6">
          <DedupSettings 
            v-model:strategy="strategy"
            :isDark="isDark"
            class="transition-all duration-500 ease-in-out shadow-app-dark"
          />
        </div>
      </div>
    </main>
  </div>
</template>

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
const isDark = ref(false) // Default to light theme
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

// Toggle dark mode
function toggleDarkMode() {
  isDark.value = !isDark.value;
  document.documentElement.classList.toggle('dark', isDark.value);
}

// Initialize editor
const editor = useEditor({
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Paste your text here...'
    })
  ],
  content: '',
  editorProps: {
    attributes: {
      class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl focus:outline-none w-full max-w-none'
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
const addTestText = () => {
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

<style lang="postcss">
.ProseMirror {
  @apply min-h-[200px] font-mono;
}

.ProseMirror p {
  @apply my-0;
}

.ProseMirror p.is-editor-empty:first-child::before {
  @apply text-gray-400 float-left h-0 pointer-events-none;
  content: attr(data-placeholder);
}

.dark .ProseMirror p.is-editor-empty:first-child::before {
  @apply text-gray-500;
}

/* Custom scrollbar for Webkit browsers */
::-webkit-scrollbar {
  @apply w-2;
}

::-webkit-scrollbar-track {
  @apply bg-transparent;
}

::-webkit-scrollbar-thumb {
  @apply bg-gray-300 rounded-full hover:bg-gray-400 transition-colors duration-200;
}

.dark ::-webkit-scrollbar-thumb {
  @apply bg-gray-700 hover:bg-gray-600;
}
</style>