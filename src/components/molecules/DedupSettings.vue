<!-- DedupSettings.vue -->
<script setup lang="ts">
import { defineEmits, defineProps, ref, computed } from 'vue'
import { type DedupStrategy, SplitStrategy, ComparisonScope, SimilarityMethod } from '../../types/dedup'
import Text from '../atoms/Text.vue'
import Switch from '../atoms/Switch.vue'

interface Props {
  strategy: DedupStrategy
  isDark?: boolean
}

const emit = defineEmits<{
  (e: 'update:strategy', value: Props['strategy']): void;
}>();

const props = defineProps<Props>()

// Track active preset
const activePreset = ref<string>("Similar Ideas")

function handleChange<K extends keyof DedupStrategy>(key: K, value: DedupStrategy[K]) {
  emit('update:strategy', {
    ...props.strategy,
    [key]: value
  })
}

// Settings options
const splitStrategies = [
  { id: SplitStrategy.Characters, name: 'Characters' },
  { id: SplitStrategy.Words, name: 'Words' },
  { id: SplitStrategy.Sentences, name: 'Sentences' },
  { id: SplitStrategy.Paragraphs, name: 'Paragraphs' },
  { id: SplitStrategy.WholeText, name: 'Whole Text' },
]

const comparisonScopes = [
  { id: ComparisonScope.Local, name: 'Local' },
  { id: ComparisonScope.Global, name: 'Global' },
]

const similarityMethods = [
  { id: SimilarityMethod.Exact, name: 'Exact' },
  { id: SimilarityMethod.Fuzzy, name: 'Fuzzy' },
  { id: SimilarityMethod.Semantic, name: 'Semantic' },
]

// Define the type for presets
interface Preset {
  name: string;
  description: string;
  config: Partial<DedupStrategy>;
}

const presets: Preset[] = [
  {
    name: "Perfect Match",
    description: "Find exact copies of text, including spaces and punctuation",
    config: {
      case_sensitive: true,
      ignore_whitespace: false,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.WholeText,
      comparison_scope: ComparisonScope.Global,
      min_length: 1,
      similarity_threshold: 1.0,
      similarity_method: SimilarityMethod.Exact
    }
  },
  {
    name: "Almost Identical",
    description: "Find text that's nearly the same, ignoring small differences",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: false,
      normalize_unicode: true,
      split_strategy: SplitStrategy.WholeText,
      comparison_scope: ComparisonScope.Global,
      min_length: 3,
      similarity_threshold: 0.95,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Similar Ideas",
    description: "Find sentences that express the same thoughts, even if worded differently",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Sentences,
      comparison_scope: ComparisonScope.Global,
      min_length: 3,
      similarity_threshold: 0.8,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Related Paragraphs",
    description: "Find paragraphs that cover similar topics or points",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Paragraphs,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.7,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Similar Wording",
    description: "Find text that uses the same words, even in different order",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Words,
      comparison_scope: ComparisonScope.Global,
      min_length: 3,
      similarity_threshold: 0.75,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Close Spelling",
    description: "Find text that's spelled almost the same way, catching typos",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Characters,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.85,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Same Meaning",
    description: "Find text that means the same thing, even if using different words",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Sentences,
      comparison_scope: ComparisonScope.Global,
      min_length: 5,
      similarity_threshold: 0.7,
      similarity_method: SimilarityMethod.Semantic
    }
  },
  {
    name: "Nearby Matches",
    description: "Find similar text that appears close together in the document",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Sentences,
      comparison_scope: ComparisonScope.Local,
      min_length: 3,
      similarity_threshold: 0.8,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Code Duplicates",
    description: "Find similar code blocks, paying attention to spacing and symbols",
    config: {
      case_sensitive: true,
      ignore_whitespace: true,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Sentences,
      comparison_scope: ComparisonScope.Global,
      min_length: 2,
      similarity_threshold: 0.9,
      similarity_method: SimilarityMethod.Fuzzy
    }
  },
  {
    name: "Deep Meaning",
    description: "Find text that covers the same topic in detail, even with completely different wording",
    config: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.WholeText,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.9,
      similarity_method: SimilarityMethod.Semantic
    }
  }
];

const applyPreset = (preset: Preset) => {
  activePreset.value = preset.name;
  const newStrategy = { ...props.strategy, ...preset.config };
  emit('update:strategy', newStrategy);
}

// Apply Similar Ideas preset by default
if (props.strategy.similarity_threshold === 1.0) {
  applyPreset(presets[2]); // Similar Ideas is at index 2
}

const toggles = [
  { key: 'case_sensitive' as keyof DedupStrategy, label: 'Case Sensitive' },
  { key: 'ignore_whitespace' as keyof DedupStrategy, label: 'Ignore Whitespace' },
  { key: 'ignore_punctuation' as keyof DedupStrategy, label: 'Ignore Punctuation' },
  { key: 'normalize_unicode' as keyof DedupStrategy, label: 'Normalize Unicode' },
] as const

const showAdvanced = ref(false)

const sliderStyle = computed(() => ({
  '--value-percent': props.strategy.similarity_threshold
}))
</script>

<template>
  <div class="card max-w-full overflow-hidden">
    <div class="card-header">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <h3 class="font-medium text-sm text-theme-primary">
            Deduplication Settings
          </h3>
        </div>
      </div>
    </div>

    <div class="p-4 space-y-6">
      <!-- Presets -->
      <div class="space-y-4">
        <div>
          <h4 class="text-sm font-medium text-theme-primary">Presets</h4>
          <p class="text-xs text-theme-secondary mt-0.5">
            Select a preset to quickly adjust the deduplication settings
          </p>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <button
            v-for="preset in presets"
            :key="preset.name"
            @click="applyPreset(preset)"
            class="preset-button"
            :class="[
              Math.abs(props.strategy.similarity_threshold - (preset.config.similarity_threshold ?? 1.0)) < 0.05
                ? 'preset-button-active'
                : 'preset-button-inactive'
            ]"
          >
            <div class="font-medium">{{ preset.name }}</div>
            <div class="text-xs mt-1 opacity-90">{{ preset.description }}</div>
          </button>
        </div>
      </div>

      <!-- Similarity Slider -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h4 class="text-sm font-medium text-theme-primary">Similarity Threshold</h4>
            <p class="text-xs text-theme-secondary mt-0.5">
              Adjust how similar text needs to be to be considered a duplicate
            </p>
          </div>
          <span class="text-sm font-medium text-theme-primary">
            {{ Math.round(props.strategy.similarity_threshold * 100) }}%
          </span>
        </div>
        
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          v-model="props.strategy.similarity_threshold"
          @input="$emit('update:strategy', { ...props.strategy })"
          class="w-full h-2 bg-theme-bg-elevated-light dark:bg-theme-bg-elevated-dark rounded-full appearance-none cursor-pointer"
        />
      </div>

      <!-- Advanced Settings Toggle -->
      <button 
        @click="showAdvanced = !showAdvanced"
        class="w-full flex items-center justify-between text-xs font-medium px-3 py-2 rounded-lg transition-all duration-300"
        :class="{
          'bg-gray-700/30 text-gray-400 hover:text-gray-300 hover:bg-gray-700/40': props.isDark,
          'bg-gray-100 text-gray-600 hover:bg-gray-200': !props.isDark
        }"
      >
        <span>Advanced Settings</span>
        <svg class="w-4 h-4 transition-transform duration-300" :class="{ 'rotate-180': showAdvanced }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      <div v-if="showAdvanced" class="pt-2 space-y-4">
        <!-- Split Strategy -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-theme-primary">Split Strategy</h4>
          <p class="text-xs text-theme-secondary mt-0.5">
            Select how to split the text into smaller parts for comparison
          </p>
          <div class="grid grid-cols-3 gap-1">
            <button
              v-for="strategy in splitStrategies"
              :key="strategy.id"
              @click="handleChange('split_strategy', strategy.id)"
              class="px-3 py-1.5 text-xs font-medium rounded-lg transition-all duration-300"
              :class="{
                'bg-indigo-500/15 text-indigo-400 ring-1 ring-indigo-400/30': props.isDark && props.strategy.split_strategy === strategy.id,
                'bg-gray-700/30 text-gray-400 hover:text-gray-300 hover:bg-gray-700/40': props.isDark && props.strategy.split_strategy !== strategy.id,
                'bg-gradient-to-r from-indigo-600 to-indigo-700 text-white shadow-lg shadow-indigo-500/25': !props.isDark && props.strategy.split_strategy === strategy.id,
                'bg-gray-100 text-gray-600 hover:bg-gray-200': !props.isDark && props.strategy.split_strategy !== strategy.id
              }"
            >
              {{ strategy.name }}
            </button>
          </div>
        </div>

        <!-- Toggles -->
        <div class="space-y-4">
          <div class="flex items-center justify-between py-1">
            <span class="text-xs" :class="{
              'text-gray-400': props.isDark,
              'text-gray-600': !props.isDark
            }">Case Sensitive</span>
            <Switch v-model="props.strategy.case_sensitive" />
          </div>
          <div class="flex items-center justify-between py-1">
            <span class="text-xs" :class="{
              'text-gray-400': props.isDark,
              'text-gray-600': !props.isDark
            }">Ignore Whitespace</span>
            <Switch v-model="props.strategy.ignore_whitespace" />
          </div>
          <div class="flex items-center justify-between py-1">
            <span class="text-xs" :class="{
              'text-gray-400': props.isDark,
              'text-gray-600': !props.isDark
            }">Ignore Punctuation</span>
            <Switch v-model="props.strategy.ignore_punctuation" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
input[type="range"] {
  -webkit-appearance: none;
  width: 100%;
  height: 4px;
  border-radius: 2px;
  outline: none;
}

input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.15s ease-in-out;
}

.slider-dark {
  background: #282B33;
}

.slider-dark::-webkit-slider-thumb {
  background: #6366F1;
  border: 2px solid #1E2128;
}

.slider-dark::-webkit-slider-thumb:hover {
  background: #818CF8;
}

.slider-light {
  background: #E5E7EB;
}

.slider-light::-webkit-slider-thumb {
  background: #6366F1;
  border: 2px solid white;
}

.slider-light::-webkit-slider-thumb:hover {
  background: #818CF8;
}

.preset-button {
  @apply py-3 px-4 rounded-lg text-left transition-all duration-200 hover:shadow-md;
}

.preset-button-active {
  @apply bg-brand-primary text-white shadow-sm;
}

.preset-button-inactive {
  @apply bg-theme-bg-surface-light dark:bg-theme-bg-surface-dark text-theme-text-primary-light dark:text-theme-text-primary-dark hover:bg-theme-bg-elevated-light dark:hover:bg-theme-bg-elevated-dark;
}

.card {
  @apply bg-theme-bg-base-light dark:bg-theme-bg-base-dark rounded-xl border border-theme-border-light dark:border-theme-border-dark shadow-surface-light dark:shadow-surface-dark;
}

.card-header {
  @apply p-4 border-b border-theme-border-light dark:border-theme-border-dark;
}
</style>
