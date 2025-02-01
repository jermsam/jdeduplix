import {onMounted, ref, watch} from 'vue';
import { invoke } from '@tauri-apps/api/core'
import {DedupStrategy, DEFAULT_STRATEGY, DuplicateGroup} from '../types/dedup.ts';


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
          caseSensitive: newStrategy.case_sensitive,
          ignoreWhitespace: newStrategy.ignore_whitespace,
          ignorePunctuation: newStrategy.ignore_punctuation,
          normalizeUnicode: newStrategy.normalize_unicode,
          splitStrategy: newStrategy.split_strategy,
          comparisonScope: newStrategy.comparison_scope,
          minLength: newStrategy.min_length,
          similarityThreshold: newStrategy.similarity_threshold,
          similarityMethod: newStrategy.similarity_method,
          useParallel: newStrategy.use_parallel,
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
      const savedStrategy = await invoke<string>('get_strategy')
      if (savedStrategy) {
        strategy.value = JSON.parse(savedStrategy)
      }
    } catch (error) {
      console.error('Failed to load saved strategy:', error)
    }
  }

  const findDuplicates = async (text: string) => {
    try {
      await loadSavedStrategy()
      // If we're updating the strategy, wait for it to finish
      if (isUpdatingStrategy.value) {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      
      // First add the text - use named parameter
      await invoke<number>('add_text', { text })
      texts.value.push(text)
      
      // If we're using semantic similarity, wait a bit for processing
      if (strategy.value.similarity_method === 'Semantic') {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      
      // Then get the duplicates
      const results = await invoke<number[][]>('deduplicate_texts')
      
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

  onMounted(async ()=>{
    // Load strategy from backend
    await loadSavedStrategy()
  })

  return {
    strategy,
    duplicates,
    loadSavedStrategy,
    findDuplicates,
    clearDuplicates,
  }
}
