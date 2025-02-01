<template>
  <div class="relative group">
    <div class="absolute inset-0 bg-gradient-to-r from-indigo-500/10 to-purple-500/10 dark:from-indigo-500/5 dark:to-purple-500/5 rounded-2xl blur-2xl transition-opacity duration-500 opacity-0 group-hover:opacity-100"></div>
    <div class="relative">
      <div class="absolute top-3 inset-x-3 flex items-center justify-between z-10">
        <div class="flex items-center space-x-3">
          <div class="flex space-x-1.5 p-2 bg-slate-100 dark:bg-slate-800 rounded-lg shadow-sm">
            <div class="w-3 h-3 rounded-full bg-red-400 dark:bg-red-500" />
            <div class="w-3 h-3 rounded-full bg-amber-400 dark:bg-amber-500" />
            <div class="w-3 h-3 rounded-full bg-green-400 dark:bg-green-500" />
          </div>
          <span class="text-sm font-semibold text-slate-700 dark:text-slate-200">Editor</span>
        </div>
        <div class="flex items-center divide-x divide-slate-200 dark:divide-slate-700 bg-slate-100 dark:bg-slate-800 rounded-lg shadow-sm">
          <button
              @click="addTestText"
              class="px-3 py-1.5 text-xs font-medium text-slate-600 dark:text-slate-300 hover:text-indigo-500 dark:hover:text-indigo-400 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors"
          >
            Add Test Text
          </button>
          <button
              @click="$emit('process', text )"
              :disabled="!hasText"
              class="px-4 py-1.5 text-xs font-semibold text-white bg-gradient-to-r from-indigo-500 to-purple-500 dark:from-indigo-400 dark:to-purple-400 hover:from-indigo-600 hover:to-purple-600 dark:hover:from-indigo-500 dark:hover:to-purple-500 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm transition-all hover:-translate-y-0.5 hover:shadow-md hover:shadow-indigo-500/25 dark:hover:shadow-indigo-400/25"
          >
            Process
          </button>
          <button
              @click="clearText"
              :disabled="!hasText"
              class="px-3 py-1.5 text-xs font-medium text-slate-600 dark:text-slate-300 hover:text-indigo-500 dark:hover:text-indigo-400 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors"
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
</template>

<script setup lang="ts">
import { watch, onMounted, computed } from 'vue'
import { useEditor, EditorContent, } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'

const props = defineProps<{
  modelValue: string
  isProcessing: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'process', value: string): void
}>()

const editor = useEditor({
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Paste your text here to find duplicates...',
    }),
  ],
  editorProps: {
    attributes: {
      class: 'w-full h-80 pt-12 px-5 pb-5 bg-white dark:bg-gray-900 rounded-2xl ring-2 ring-dashed ring-slate-300/70 dark:ring-gray-700/70 hover:ring-indigo-500/50 dark:hover:ring-indigo-400/50 focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 focus:outline-none resize-none text-slate-700 dark:text-gray-100 placeholder-slate-400 dark:placeholder-gray-500 transition-all duration-200 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:repeating-linear-gradient(135deg,rgba(99,102,241,0.012)_0px,rgba(99,102,241,0.012)_2px,transparent_2px,transparent_4px)] dark:[background-image:repeating-linear-gradient(135deg,rgba(129,140,248,0.012)_0px,rgba(129,140,248,0.012)_2px,transparent_2px,transparent_4px)] [background-size:4px_4px] prose prose-sm max-w-none prose-slate dark:prose-invert prose-p:my-0 prose-headings:my-0',
    },
  },
  autofocus: 'end',
})

const text = computed(()=>{
  return (editor.value?.getText() || '').trim();
})

const hasText = computed(() => text.value.length > 0)

// Sync editor content with modelValue
watch(editor, () => {
  if (editor.value) {
    emit('update:modelValue', editor.value.getText())
  }
}, { deep: true })

// Update editor when modelValue changes externally
watch(() => props.modelValue, (newValue) => {
  const editorContent = editor.value?.getText()
  if (editor.value && newValue !== editorContent) {
    editor.value.commands.setContent(newValue)
  }
})

onMounted(() => {
  // Focus editor when component mounts
  setTimeout(() => {
    editor.value?.commands.focus('end')
  }, 100)
})

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
</script>
