<template>
  <div class="duplicate-results">
  
    <div v-if="duplicates.length === 0" class=" no-results mt-8 flex flex-col items-center justify-center text-center">
            <CheckCircleIcon class="w-12 h-12 text-green-500 dark:text-green-400" />
            <h3 class="mt-2 text-sm font-medium text-slate-900 dark:text-slate-100">No Duplicates Found</h3>
            <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">Your text appears to be free of duplicate content.</p>
    </div>
    <div v-else class="results-container">
      <div class="space-y-4">
        <!-- Header -->
        <div class="flex items-center justify-between">
          <div class="text-sm font-medium text-slate-700 dark:text-gray-100">Duplicate Results</div>
          <div class="text-xs text-slate-500 dark:text-gray-400">{{ duplicates.length }} duplicates found</div>
        </div>

        <!-- Results -->
        <div class="space-y-4">
          <div v-for="(group, index) in duplicates" :key="index" class="bg-white dark:bg-gray-900 rounded-lg shadow-sm ring-1 ring-slate-200/80 dark:ring-gray-800 p-4">
            <div class="flex items-center justify-between mb-2">
              <div class="text-sm font-medium text-slate-700 dark:text-gray-100">Duplicate {{ index + 1 }}</div>
              <div class="text-xs text-slate-500 dark:text-gray-400">{{ group.similarity.toFixed(1) }}% similar</div>
            </div>
            <div class="text-sm text-slate-600 dark:text-gray-300 whitespace-pre-wrap">{{ group.content }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CheckCircleIcon } from '@heroicons/vue/24/outline'
interface DuplicateGroup {
  content: string
  similarity: number
}

const props = defineProps<{
  duplicates: DuplicateGroup[]
}>()
</script>

<style scoped>
.duplicate-results {
  padding: 1rem;
}

.no-results {
  text-align: center;
  padding: 2rem;
  color: #666;
}

.duplicate-group {
  margin-bottom: 2rem;
  background: #f5f5f5;
  border-radius: 8px;
  padding: 1rem;
}

.duplicate-group h3 {
  margin-bottom: 1rem;
  color: #333;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: white;
  border-radius: 4px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.file-name {
  font-weight: 500;
}

.file-path {
  font-size: 0.875rem;
  color: #666;
}

.file-size {
  font-size: 0.875rem;
  color: #888;
}

.file-actions {
  display: flex;
  gap: 0.5rem;
}

button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background-color 0.2s;
}

.preview-btn {
  background-color: #e0e0e0;
  color: #333;
}

.preview-btn:hover {
  background-color: #d0d0d0;
}

.delete-btn {
  background-color: #ff4444;
  color: white;
}

.delete-btn:hover {
  background-color: #ff2222;
}
</style>
