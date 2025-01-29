import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { DedupStrategy, DuplicateGroup, DEFAULT_STRATEGY } from '../types/dedup'

export function useDeduplication() {
  const strategy = ref<DedupStrategy>({ ...DEFAULT_STRATEGY })
  const duplicates = ref<DuplicateGroup[]>([])

  // Watch for strategy changes and save to backend
  watch(strategy, async (newStrategy) => {
    if (newStrategy) {
      try {
        await invoke('update_strategy', { strategy: newStrategy })
      } catch (error) {
        console.error('Failed to save strategy:', error)
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
      // First process the text
      await invoke('process_text', { content: text })
      
      // Then get the duplicates
      const results = await invoke<string[][]>('get_duplicates')
      
      // Transform the results into DuplicateGroup format
      duplicates.value = results.map(group => ({
        original: group[0],
        duplicates: group.slice(1),
        similarity: strategy.value.similarity_threshold
      }))
    } catch (error) {
      console.error('Failed to find duplicates:', error)
      duplicates.value = []
    }
  }

  const clearDuplicates = () => {
    duplicates.value = []
  }

  // Load strategy from backend
  loadSavedStrategy()

  return {
    strategy,
    duplicates,
    loadSavedStrategy,
    findDuplicates,
    clearDuplicates
  }
}
