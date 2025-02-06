<!-- DedupSettings.vue -->
<script setup lang="ts">
  import Slider from '../atoms/Slider.vue';
  import Input from '../atoms/Input.vue';
  import Switch from '../atoms/Switch.vue';
  import Select from '../atoms/Select.vue';
  import {DEDUP_PRESETS} from '../../types/dedup.ts';
  import {SplitStrategy, ComparisonScope, FuzzyAlgorithm} from '../../types/enums';
  import {type DedupStrategyType, type DedupPresetType} from '../../types/dedup.ts';
  import {ref, watch, computed} from 'vue';

  const props = defineProps<{
    strategy: DedupStrategyType
    isDark?: boolean
  }>();

  const emit = defineEmits<{
    (e: 'update:strategy', value: DedupStrategyType): void
  }>();

  const presets = DEDUP_PRESETS;
  const selectedPreset = ref<DedupPresetType>();
  const showAdvancedSettings = ref(false);

  const currentSettings = computed(() => {
    return selectedPreset.value?.settings  || ( props.strategy || presets[0].settings);
  });

  watch(() => props.strategy, (newVal, oldVal) => {
    if (newVal && newVal !== oldVal) {

      const matchingPreset = presets.find((preset) => JSON.stringify(preset.settings) === JSON.stringify(newVal));
      if (matchingPreset) {
        selectedPreset.value = matchingPreset;
      }
    }
  }, {immediate: true, deep: true});

  function updateStrategy(key: keyof DedupStrategyType, value: any) {
    emit('update:strategy', {
      ...currentSettings.value,
      [key]: value,
    });
  }

  const splitStrategyOptions = Object.values(SplitStrategy);
  const comparisonScopeOptions = Object.values(ComparisonScope);
  const similarityMethodOptions = ["Exact", "Semantic", "Levenshtein", "Fuzzy"];
  const fuzzyAlgorithmOptions = Object.values(FuzzyAlgorithm).map(val => val as string);
</script>

<template>
  <div class="space-y-6">
    <!-- Presets Section -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div class="text-sm font-medium text-slate-800 dark:text-gray-100">Matching Presets</div>
        <div class="text-xs text-slate-500 dark:text-gray-400">Select how you want to find duplicates</div>
      </div>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <button
            v-for="preset in presets"
            :key="preset.name"
            @click="selectedPreset = preset; emit('update:strategy', {...preset.settings})"
            :class="[
              'group relative p-4 text-left text-gray-600 dark:text-gray-100 bg-white/95 dark:bg-gray-900/95 rounded-xl ring-1 transition-all duration-300 ease-[cubic-bezier(.2,.8,.4,1)] transform hover:scale-[1.008] hover:-translate-y-[2px] shadow-[0_1px_3px_-1px_rgba(0,0,0,0.02),0_0_2px_0_rgba(99,102,241,0.05)] dark:shadow-[0_2px_4px_-2px_rgba(0,0,0,0.3),0_0_3px_0_rgba(129,140,248,0.15)] backdrop-blur-[8px] backdrop-saturate-[1.3]',
              selectedPreset?.name === preset.name 
                ? 'ring-indigo-500 dark:ring-indigo-400' 
                : 'ring-black/[0.04] dark:ring-white/[0.04] hover:ring-black/[0.06] dark:hover:ring-white/[0.06]'
            ]"
        >
          <div class="relative z-10">
            <h3 class="font-medium mb-1">{{ preset.name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ preset.description }}</p>
          </div>
          <div class="absolute inset-0 rounded-xl bg-gradient-to-br pointer-events-none transition-opacity duration-300"
               :class="[
                 selectedPreset?.name === preset.name 
                   ? 'from-indigo-50/50 to-transparent dark:from-indigo-500/[0.12] dark:to-transparent opacity-100' 
                   : 'from-indigo-50/0 to-indigo-50/0 group-hover:from-indigo-50/50 group-hover:to-transparent dark:group-hover:from-indigo-500/[0.12] dark:group-hover:to-transparent opacity-0 group-hover:opacity-100'
               ]"
          ></div>
        </button>
      </div>
    </div>

    <!-- Advanced Settings Toggle -->
    <div class="flex items-center justify-between px-4 py-2 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Advanced Settings</span>
      <button 
        @click="showAdvancedSettings = !showAdvancedSettings"
        class="text-sm text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300"
      >
        {{ showAdvancedSettings ? 'Hide' : 'Show' }}
      </button>
    </div>

    <!-- Advanced Settings Panel -->
    <div v-if="showAdvancedSettings" class="space-y-6 bg-white dark:bg-gray-900 rounded-xl ring-1 ring-slate-200 dark:ring-gray-800 p-6 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:radial-gradient(rgba(0,0,0,0.015)_1px,transparent_1px)] dark:[background-image:radial-gradient(rgba(255,255,255,0.015)_1px,transparent_1px)] [background-size:16px_16px]">
      <!-- Text Processing -->
      <div class="space-y-4">
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100">Text Processing</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Switch
            v-for="(value, key) in {
              case_sensitive: 'Case Sensitive',
              ignore_whitespace: 'Ignore Whitespace',
              ignore_punctuation: 'Ignore Punctuation',
              normalize_unicode: 'Normalize Unicode'
            }"
            :key="key"
            :label="value"
            :model-value="currentSettings[key]"
            @update:model-value="updateStrategy(key, $event)"
          />
        </div>
      </div>

      <!-- Analysis Settings -->
      <div class="space-y-4">
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100">Analysis Settings</h3>
        <div class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Select
              label="Split Strategy"
              :model-value="currentSettings.split_strategy"
              @update:model-value="updateStrategy('split_strategy', $event)"
              :options="splitStrategyOptions"
            />
            <Select
              label="Comparison Scope"
              :model-value="currentSettings.comparison_scope"
              @update:model-value="updateStrategy('comparison_scope', $event)"
              :options="comparisonScopeOptions"
            />
          </div>

          <Select
            label="Similarity Method"
            :model-value="currentSettings.similarity_method.type"
            @update:model-value="updateStrategy('similarity_method', { ...currentSettings.similarity_method, type: $event })"
            :options="similarityMethodOptions"
          />

          <!-- Fuzzy Algorithm Selection -->
          <Select
            v-if="currentSettings.similarity_method.type === 'Fuzzy'"
            label="Fuzzy Algorithm"
            :model-value="currentSettings.similarity_method.algorithm || FuzzyAlgorithm.DamerauLevenshtein"
            @update:model-value="updateStrategy('similarity_method', {...currentSettings.similarity_method,  algorithm: $event })"
            :options="fuzzyAlgorithmOptions"
          />

          <Input
            type="number"
            label="Minimum Length"
            :model-value="currentSettings.min_length"
            @update:model-value="updateStrategy('min_length', parseInt($event))"
            min="1"
            max="50"
          />

          <div class="space-y-2">
            <Slider
            label="Similarity Threshold"
              :model-value="currentSettings.similarity_threshold"
              @update:model-value="updateStrategy('similarity_threshold',$event)"
              :min="0"
              :max="100"
              :step="1"
            />
          </div>
        </div>
      </div>

      <!-- Additional Options -->
      <div class="space-y-4">
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100">Additional Options</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Switch
            v-for="(value, key) in {
              use_parallel: 'Use Parallel Processing',
              ignore_stopwords: 'Ignore Stop Words',
              stemming: 'Use Word Stemming',
              language_detection: 'Language Detection'
            }"
            :key="key"
            :label="value"
            :model-value="currentSettings[key]"
            @update:model-value="updateStrategy(key, $event)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Keep any custom styles needed for components not covered by Input atom */
</style>
