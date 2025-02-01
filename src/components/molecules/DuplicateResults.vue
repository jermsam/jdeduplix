<template>
  <div class="duplicate-results">
    <div v-if="duplicates.length === 0" class="no-results">
      <p>No duplicates found yet. Start a scan to find duplicate files.</p>
    </div>
    <div v-else class="results-container">
      <div v-for="(group, index) in duplicates" :key="index" class="duplicate-group">
        <h3>Duplicate Group {{ index + 1 }}</h3>
        <div class="file-list">
          <div v-for="file in group" :key="file.path" class="file-item">
            <div class="file-info">
              <span class="file-name">{{ file.name }}</span>
              <span class="file-path">{{ file.path }}</span>
              <span class="file-size">{{ formatFileSize(file.size) }}</span>
            </div>
            <div class="file-actions">
              <button @click="previewFile(file)" class="preview-btn">Preview</button>
              <button @click="deleteFile(file)" class="delete-btn">Delete</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface FileInfo {
  name: string
  path: string
  size: number
}

const props = defineProps<{
  duplicates: FileInfo[][]
}>()

const emit = defineEmits<{
  (e: 'delete', file: FileInfo): void
  (e: 'preview', file: FileInfo): void
}>()

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

function deleteFile(file: FileInfo) {
  emit('delete', file)
}

function previewFile(file: FileInfo) {
  emit('preview', file)
}
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
