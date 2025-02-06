<template>
  <div class="duplicate-results">

    <div v-if="totalDuplicates === 0"
         class=" no-results mt-8 flex flex-col items-center justify-center text-center">
      <CheckCircleIcon class="w-12 h-12 text-green-500 dark:text-green-400"/>
      <h3 class="mt-2 text-sm font-medium text-slate-900 dark:text-slate-100">No Duplicates Found</h3>
      <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">Your text appears to be free of duplicate content.</p>
    </div>
    <div v-else class="results-container">
      <div class="space-y-4">
        <!-- Header -->
        <div class="flex items-center justify-between">
          <div class="text-sm font-medium text-slate-700 dark:text-gray-100">Duplicate Results</div>
          <div class="text-xs text-slate-500 dark:text-gray-400">{{ totalDuplicates }} {{ totalDuplicates === 1 ? 'duplicate' : 'duplicates' }} found</div>
        </div>

        <!-- Results -->
        <div class="space-y-4">
          <div v-for="(group, index) in duplicateGroups" :key="index"
               class="bg-white dark:bg-gray-900 rounded-lg shadow-sm ring-1 ring-slate-200/80 dark:ring-gray-800 p-4">
            <div class="flex items-center justify-between mb-2">
              <div class="text-sm font-medium text-slate-700 dark:text-gray-100">Duplicate Text</div>
              <div class="text-xs text-slate-500 dark:text-gray-400">
                Appears {{ group.duplicates.length + 1 }} times
              </div>
            </div>
            <!-- The duplicate text -->
            <div class="text-sm text-slate-600 dark:text-gray-300 whitespace-pre-wrap bg-slate-50 dark:bg-gray-800 p-2 rounded">
              {{ group.original }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import {CheckCircleIcon} from '@heroicons/vue/24/outline';
  import {DuplicateGroupType} from '../../types/dedup.ts';
  import {computed} from 'vue';

  const props = defineProps<{
    duplicateGroups: DuplicateGroupType[]
  }>();

  const totalDuplicates = computed(() => {
    return props.duplicateGroups.reduce((total, group) => {
      total += group.duplicates.length;
      return total;
    }, 0);
  });

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

  .duplicate-group h3 {
    margin-bottom: 1rem;
    color: #333;
  }


  button {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    border: none;
    cursor: pointer;
    font-size: 0.875rem;
    transition: background-color 0.2s;
  }


</style>
