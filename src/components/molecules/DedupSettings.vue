<!-- DedupSettings.vue -->
<script setup lang="ts">
import { defineEmits, defineProps, ref, computed } from 'vue'
import { type DedupStrategy, SplitStrategy, ComparisonScope, SimilarityMethod } from '../../types/dedup'
import Text from '../atoms/Text.vue'
import Switch from '../atoms/Switch.vue'

const props = defineProps<{
  strategy: DedupStrategy
  isDark?: boolean
}>()

const emit = defineEmits<{
  'update:strategy': [strategy: DedupStrategy]
}>()

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
  <div class="bg-[#1A1D23] rounded-lg overflow-hidden font-sans">
    <!-- Main Settings -->
    <div class="p-3">
      <!-- Presets -->
      <div class="mb-3">
        <h3 class="text-xs font-medium text-gray-400 mb-2">Choose a Preset</h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
          <button
            v-for="preset in presets"
            :key="preset.name"
            @click="applyPreset(preset)"
            :class="[
              'p-2 rounded-lg text-left hover:bg-[#23262E] transition-colors',
              activePreset === preset.name ? 'bg-[#23262E] ring-1 ring-indigo-500' : 'bg-[#1E2128]'
            ]"
          >
            <div class="text-xs font-medium leading-snug text-gray-200">{{ preset.name }}</div>
            <div class="text-xs leading-relaxed text-gray-400 mt-0.5">{{ preset.description }}</div>
          </button>
        </div>
      </div>

      <!-- Similarity -->
      <div class="mb-3">
        <div class="flex items-center justify-between mb-2">
          <div class="text-xs font-medium text-gray-300">Similarity</div>
          <div class="text-xs font-medium text-gray-400">{{ Math.round(props.strategy.similarity_threshold * 100) }}%</div>
        </div>
        <input
          type="range"
          min="0"
          max="1"
          step="0.05"
          :value="props.strategy.similarity_threshold"
          :style="sliderStyle"
          class="w-full"
          @input="(e) => handleChange('similarity_threshold', parseFloat((e.target as HTMLInputElement).value))"
        />
      </div>
    </div>

    <!-- Advanced Settings Toggle -->
    <button 
      @click="showAdvanced = !showAdvanced"
      class="w-full p-2 border-t border-gray-800 flex items-center justify-between hover:bg-[#1E2128] transition-colors"
    >
      <span class="text-xs font-medium text-gray-300">Advanced Settings</span>
      <div 
        class="w-4 h-4 rounded-full bg-[#1E2128] flex items-center justify-center transition-transform"
        :class="{ 'rotate-180': showAdvanced }"
      >
        <svg class="w-3 h-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </div>
    </button>

    <!-- Advanced Settings Content -->
    <div v-if="showAdvanced" class="border-t border-gray-800">
      <div class="p-3 space-y-3">
        <!-- Toggles -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
          <div 
            v-for="toggle in toggles" 
            :key="toggle.key"
            class="flex items-center justify-between p-2 rounded-lg bg-[#1E2128]"
          >
            <span class="text-xs font-medium text-gray-300">{{ toggle.label }}</span>
            <Switch
              :model-value="props.strategy[toggle.key] as boolean"
              @update:model-value="(value) => handleChange(toggle.key, value)"
            />
          </div>
        </div>

        <!-- Split Strategy -->
        <div>
          <div class="text-xs font-medium text-gray-300 mb-2">Split Strategy</div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <button
              v-for="strategy in splitStrategies"
              :key="strategy.id"
              @click="handleChange('split_strategy', strategy.id)"
              :class="[
                'p-2 rounded-lg text-xs transition-colors text-center',
                props.strategy.split_strategy === strategy.id
                  ? 'bg-indigo-500 text-white'
                  : 'bg-[#1E2128] text-gray-300 hover:bg-[#23262E]'
              ]"
            >
              {{ strategy.name }}
            </button>
          </div>
        </div>

        <!-- Method -->
        <div>
          <div class="text-xs font-medium text-gray-300 mb-2">Method</div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <button
              v-for="method in similarityMethods"
              :key="method.id"
              @click="handleChange('similarity_method', method.id)"
              :class="[
                'p-2 rounded-lg text-xs transition-colors text-center',
                props.strategy.similarity_method === method.id
                  ? 'bg-indigo-500 text-white'
                  : 'bg-[#1E2128] text-gray-300 hover:bg-[#23262E]'
              ]"
            >
              {{ method.name }}
            </button>
          </div>
        </div>

        <!-- Scope -->
        <div>
          <div class="text-xs font-medium text-gray-300 mb-2">Scope</div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <button
              v-for="scope in comparisonScopes"
              :key="scope.id"
              @click="handleChange('comparison_scope', scope.id)"
              :class="[
                'p-2 rounded-lg text-xs transition-colors text-center',
                props.strategy.comparison_scope === scope.id
                  ? 'bg-indigo-500 text-white'
                  : 'bg-[#1E2128] text-gray-300 hover:bg-[#23262E]'
              ]"
            >
              {{ scope.name }}
            </button>
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
  height: 6px;
  background: #1E2128;
  border-radius: 4px;
  outline: none;
}

input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  background: rgb(99 102 241); /* indigo-500 */
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid rgb(129 140 248); /* indigo-400 */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

input[type="range"]::-moz-range-thumb {
  width: 20px;
  height: 20px;
  background: rgb(99 102 241);
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid rgb(129 140 248);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

/* Color for the filled part of the slider */
input[type="range"] {
  background: linear-gradient(to right, rgb(99 102 241) 0%, rgb(99 102 241) calc(var(--value-percent) * 100%), #1E2128 calc(var(--value-percent) * 100%), #1E2128 100%);
}
</style>
