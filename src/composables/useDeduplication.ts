import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { DedupStrategy, DuplicateGroup, DEFAULT_STRATEGY } from '../types/dedup'

export function useDeduplication() {
  const strategy = ref<DedupStrategy>({ ...DEFAULT_STRATEGY })
  const duplicates = ref<DuplicateGroup[]>([])
  const texts = ref<string[]>([])
  const isUpdatingStrategy = ref(false)

  // Watch for strategy changes and save to backend
  watch(strategy, async (newStrategy) => {
    if (newStrategy) {
      try {
        isUpdatingStrategy.value = true
        await invoke('update_strategy', {
          case_sensitive: newStrategy.case_sensitive,
          ignore_whitespace: newStrategy.ignore_whitespace,
          ignore_punctuation: newStrategy.ignore_punctuation,
          normalize_unicode: newStrategy.normalize_unicode,
          split_strategy: newStrategy.split_strategy,
          comparison_scope: newStrategy.comparison_scope,
          min_length: newStrategy.min_length,
          similarity_threshold: newStrategy.similarity_threshold,
          similarity_method: newStrategy.similarity_method,
          use_parallel: newStrategy.use_parallel,
        })
        // If we're using semantic similarity, wait a bit for processing
        if (newStrategy.similarity_method === 'Semantic') {
          await new Promise(resolve => setTimeout(resolve, 100))
        }
      } catch (error) {
        console.error('Failed to save strategy:', error)
      } finally {
        isUpdatingStrategy.value = false
      }
    }
  }, { deep: true })

  const loadSavedStrategy = async () => {
    try {
      const savedStrategy = await invoke<DedupStrategy>('get_strategy')
      if (savedStrategy) {
        strategy.value = savedStrategy
      }
    } catch (error) {
      console.error('Failed to load saved strategy:', error)
    }
  }

  const findDuplicates = async (text: string) => {
    try {
      // If we're updating the strategy, wait for it to finish
      if (isUpdatingStrategy.value) {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      
      // First add the text
      const idx = await invoke<number>('add_text', { text })
      texts.value.push(text)
      
      // If we're using semantic similarity, wait a bit for processing
      if (strategy.value.similarity_method === 'Semantic') {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      
      // Then get the duplicates
      const results = await invoke<number[][]>('find_duplicates')
      
      // Transform the results into DuplicateGroup format
      duplicates.value = results.map(group => ({
        original: texts.value[group[0]],
        duplicates: group.slice(1).map(i => texts.value[i]),
        similarity: strategy.value.similarity_threshold
      }))
    } catch (error) {
      console.error('Failed to find duplicates:', error)
      duplicates.value = []
    }
  }

  const clearDuplicates = async () => {
    try {
      await invoke('clear')
      duplicates.value = []
      texts.value = []
    } catch (error) {
      console.error('Failed to clear duplicates:', error)
    }
  }

  // Load strategy from backend
  loadSavedStrategy()

  return {
    strategy,
    duplicates,
    loadSavedStrategy,
    findDuplicates,
    clearDuplicates,
  }
}
