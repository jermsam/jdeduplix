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
  <div class="min-h-screen bg-[#0F1115] text-gray-100">
    <main class="p-6">
      <Container maxWidth="2xl">
        <div class="space-y-8">
          <!-- Header -->
          <div class="flex items-center justify-between">
            <Text size="2xl" weight="bold">JDeduplix</Text>
            <div class="flex gap-2">
              <Button variant="secondary" @click="handleClear">Clear</Button>
              <Button variant="secondary" @click="addTestDuplicate">Add Test Text</Button>
              <Button variant="primary" @click="handleSubmit">Find Duplicates</Button>
            </div>
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
          <DuplicateResults :duplicates="duplicates" />
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