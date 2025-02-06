import {onMounted, ref, watch} from 'vue';
import { invoke } from '@tauri-apps/api/core'
import {
  type DedupStrategyType,
  type DuplicateResultType,  
  DEFAULT_STRATEGY, 
} from '../types/dedup.ts';


export function useDeduplication() {
  const strategy = ref<DedupStrategyType>(DEFAULT_STRATEGY)
  const results = ref<DuplicateResultType>({ duplicate_groups: [], stats: { duplicate_groups: 0, total_items: 0, unique_items: 0 } })
  const texts = ref<string[]>([])
  const isUpdatingStrategy = ref(false)

  // Function to update strategy without triggering the watcher
  const updateStrategyFromServer = (updatedStrategy: any) => {
    const parsed = {
      ...updatedStrategy,
      similarity_method: updatedStrategy.similarity_method.Fuzzy ? {
        type: 'Fuzzy',
        algorithm: updatedStrategy.similarity_method.Fuzzy
      } : {
        type: updatedStrategy.similarity_method
      }
    };
    strategy.value = parsed;
  }

  // Watch for strategy changes and save to backend
  watch(strategy, async (newStrategy, oldStrategy) => {
    if (newStrategy && JSON.stringify(newStrategy) !== JSON.stringify(oldStrategy)) {
      try {
        isUpdatingStrategy.value = true;
        console.info("🔄 Sent strategy update request");
        console.info("📥 Outgoing strategy data:", newStrategy);
        const result = await invoke<string>('update_strategy', { 
          strategy: JSON.stringify({
            ...newStrategy,
            similarity_method: newStrategy.similarity_method.type === 'Fuzzy' 
              ? { Fuzzy: newStrategy.similarity_method.algorithm }
              : newStrategy.similarity_method.type
          })
        });
        const updatedStrategy = JSON.parse(result);
        console.info("📤 Updated strategy data:", updatedStrategy);
        
        // If we're using semantic similarity, wait a bit for processing
        if (newStrategy.similarity_method.type === 'Semantic') {
          await new Promise(resolve => setTimeout(resolve, 100));
        }
      } catch (error) {
        console.error('Failed to save strategy:', error);
      } finally {
        isUpdatingStrategy.value = false;
      }
    }
  }, { deep: true,immediate:true })

  // Update initial strategy
  const loadSavedStrategy = async () => {
    try {
      isUpdatingStrategy.value = true;
      const result = await invoke<string>('get_strategy');
      const savedStrategy = JSON.parse(result);
      updateStrategyFromServer(savedStrategy);
    } catch (error) {
      console.error('Failed to load strategy:', error);
    } finally {
      isUpdatingStrategy.value = false;
    }
  };

  const findDuplicates = async (text: string) => {
    try {
      // If we're updating the strategy, wait for it to finish
      if (isUpdatingStrategy.value) {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      
      // Clear existing texts first
      await invoke('clear')
      texts.value = []
      
      // Add the new text
      await invoke<number>('add_text', { text })
      texts.value.push(text)
      
      // If we're using semantic similarity, wait a bit for processing
      if (strategy.value.similarity_method.type === 'Semantic') {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      const res = await invoke<DuplicateResultType>('deduplicate_texts');
      console.log(res);
      results.value = res;
  
    } catch (error) {
      console.error('Failed to find duplicates:', error)
      results.value = { duplicate_groups: [], stats: { duplicate_groups: 0, total_items: 0, unique_items: 0 } }
    }
  }

  const clearDuplicates = async () => {
    try {
      await invoke('clear')
      results.value = { duplicate_groups: [], stats: { duplicate_groups: 0, total_items: 0, unique_items: 0 } }
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
    results,
    loadSavedStrategy,
    findDuplicates,
    clearDuplicates,
  }
}
