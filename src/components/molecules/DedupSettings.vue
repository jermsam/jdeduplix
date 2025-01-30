<!-- DedupSettings.vue -->
<script setup lang="ts">
import { defineEmits, defineProps, ref } from 'vue'
import { type DedupStrategy, SplitStrategy, ComparisonScope, SimilarityMethod } from '../../types/dedup'
import Text from '../atoms/Text.vue'
import { ChevronUpDownIcon, CheckIcon } from '@heroicons/vue/20/solid'
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'

const props = defineProps<{
  strategy: DedupStrategy
  isDark?: boolean
}>()

const emit = defineEmits<{
  'update:strategy': [strategy: DedupStrategy]
}>()

// Track active preset
const activePreset = ref<string>("Similar Ideas")

const handleChange = <K extends keyof DedupStrategy>(key: K, value: DedupStrategy[K]) => {
  const newStrategy = {
    ...props.strategy,
    [key]: value
  }
  emit('update:strategy', newStrategy)
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


</script>

<template>
  <div class="rounded-lg overflow-hidden bg-[#1A1D23] p-4 space-y-4">
    <!-- Presets -->
    <div class="space-y-2">
      <Text size="sm" class="text-gray-400">Choose a Preset</Text>
      <div class="grid grid-cols-2 gap-4">
        <div v-for="preset in presets" :key="preset.name" 
             class="relative rounded-md p-3 cursor-pointer transition-all duration-200"
             :class="[
               activePreset === preset.name 
                 ? 'bg-blue-500/20 ring-2 ring-blue-500'
                 : 'hover:bg-gray-700/50'
             ]"
             @click="applyPreset(preset)">
          <Text size="sm" weight="medium" class="text-gray-200">{{ preset.name }}</Text>
          <Text size="xs" class="text-gray-400 mt-1">{{ preset.description }}</Text>
        </div>
      </div>
    </div>

    <!-- Quick Settings -->
    <div class="flex flex-wrap gap-3">
      <button
        @click="handleChange('case_sensitive', !props.strategy.case_sensitive)"
        :class="[
          'flex items-center gap-2 px-3 py-2 rounded-lg transition-colors',
          props.strategy.case_sensitive 
            ? 'bg-indigo-600 text-white' 
            : 'bg-[#1A1D23] text-gray-300',
          'hover:bg-[#1E2128]'
        ]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
          <path d="M13.024 9.25c.47 0 .827-.433.637-.863a4 4 0 00-4.094-2.364c-.468.05-.665.576-.43.984l1.08 1.868a.75.75 0 00.649.375h2.158zM7.84 7.758c-.236-.408-.79-.5-1.068-.12A3.982 3.982 0 006 10c0 .884.287 1.7.772 2.363.278.38.832.287 1.068-.12l1.078-1.868a.75.75 0 000-.75L7.84 7.758zM13.024 10.75H10.866a.75.75 0 00-.649.375L9.138 12.993c-.235.408-.038.934.43.984a4 4 0 004.094-2.364c.19-.43-.168-.863-.638-.863z" />
        </svg>
        Case Sensitive
      </button>

      <button
        @click="handleChange('ignore_whitespace', !props.strategy.ignore_whitespace)"
        :class="[
          'flex items-center gap-2 px-3 py-2 rounded-lg transition-colors',
          props.strategy.ignore_whitespace 
            ? 'bg-indigo-600 text-white' 
            : 'bg-[#1A1D23] text-gray-300',
          'hover:bg-[#1E2128]'
        ]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
          <path fill-rule="evenodd" d="M2 3.5A1.5 1.5 0 013.5 2h9A1.5 1.5 0 0114 3.5v11.75A2.75 2.75 0 0016.75 18h-12A2.75 2.75 0 012 15.25V3.5zm3.75 7a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zm0 3a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM5 5.75A.75.75 0 015.75 5h4.5a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.5v-.75z" clip-rule="evenodd" />
        </svg>
        Ignore Whitespace
      </button>

      <button
        @click="handleChange('ignore_punctuation', !props.strategy.ignore_punctuation)"
        :class="[
          'flex items-center gap-2 px-3 py-2 rounded-lg transition-colors',
          props.strategy.ignore_punctuation 
            ? 'bg-indigo-600 text-white' 
            : 'bg-[#1A1D23] text-gray-300',
          'hover:bg-[#1E2128]'
        ]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
          <path d="M3 3.5A1.5 1.5 0 014.5 2h6.879a1.5 1.5 0 011.06.44l4.122 4.12A1.5 1.5 0 0117 7.622V16.5a1.5 1.5 0 01-1.5 1.5h-11A1.5 1.5 0 013 16.5v-13z" />
        </svg>
        Ignore Punctuation
      </button>

      <button
        @click="handleChange('normalize_unicode', !props.strategy.normalize_unicode)"
        :class="[
          'flex items-center gap-2 px-3 py-2 rounded-lg transition-colors',
          props.strategy.normalize_unicode 
            ? 'bg-indigo-600 text-white' 
            : 'bg-[#1A1D23] text-gray-300',
          'hover:bg-[#1E2128]'
        ]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
          <path d="M3 3.5A1.5 1.5 0 014.5 2h6.879a1.5 1.5 0 011.06.44l4.122 4.12A1.5 1.5 0 0117 7.622V16.5a1.5 1.5 0 01-1.5 1.5h-11A1.5 1.5 0 013 16.5v-13z" />
        </svg>
        Normalize Unicode
      </button>
    </div>

    <!-- Similarity Threshold -->
    <div class="space-y-2">
      <div class="flex justify-between items-center">
        <Text size="sm" weight="medium" class="text-gray-300">Similarity Threshold</Text>
        <Text size="sm" class="text-gray-400">
          {{ Math.round((props.strategy.similarity_threshold || 0) * 100) }}%
        </Text>
      </div>
      <div class="relative w-full h-1.5">
        <div class="absolute inset-0 rounded-full bg-[#1A1D23]"></div>
        <div 
          class="absolute inset-y-0 left-0 bg-indigo-600 rounded-full transition-all duration-150"
          :style="{ width: `${(props.strategy.similarity_threshold || 0) * 100}%` }"
        ></div>
        <div 
          class="absolute top-1/2 -translate-y-1/2 -ml-2.5 w-5 h-5 rounded-full bg-white border-2 border-indigo-600 transition-all duration-150 pointer-events-none"
          :style="{ left: `${(props.strategy.similarity_threshold || 0) * 100}%` }"
        ></div>
        <input
          type="range"
          :value="(props.strategy.similarity_threshold || 0) * 100"
          @input="e => handleChange('similarity_threshold', Number((e.target as HTMLInputElement).value) / 100)"
          min="0"
          max="100"
          step="1"
          class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
        >
      </div>
    </div>

    <!-- Split Strategy and Comparison Scope -->
    <div class="grid grid-cols-2 gap-4">
      <!-- Split Strategy -->
      <div class="space-y-2">
        <Text size="sm" weight="medium" class="text-gray-300">Split Strategy</Text>
        <Listbox 
          :modelValue="props.strategy.split_strategy"
          @update:modelValue="value => handleChange('split_strategy', value)"
        >
          <div class="relative">
            <ListboxButton
              class="relative w-full cursor-pointer rounded-lg py-2 pl-3 pr-10 text-left bg-[#1A1D23] text-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <span class="block truncate">
                {{ splitStrategies.find(s => s.id === props.strategy.split_strategy)?.name }}
              </span>
              <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                <ChevronUpDownIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
              </span>
            </ListboxButton>

            <transition
              leave-active-class="transition duration-100 ease-in"
              leave-from-class="opacity-100"
              leave-to-class="opacity-0"
            >
              <ListboxOptions
                class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md py-1 bg-[#1A1D23] text-gray-300 focus:outline-none text-base sm:text-sm"
              >
                <ListboxOption
                  v-for="strat in splitStrategies"
                  :key="strat.id"
                  :value="strat.id"
                  v-slot="{ active, selected }"
                >
                  <li
                    :class="[
                      'relative cursor-pointer select-none py-2 pl-10 pr-4',
                      active ? 'bg-[#1E2128]' : '',
                      selected ? 'bg-indigo-600 text-white' : ''
                    ]"
                  >
                    <span :class="['block truncate', selected ? 'font-medium' : 'font-normal']">
                      {{ strat.name }}
                    </span>
                    <span
                      v-if="selected"
                      class="absolute inset-y-0 left-0 flex items-center pl-3 text-white"
                    >
                      <CheckIcon class="h-5 w-5" aria-hidden="true" />
                    </span>
                  </li>
                </ListboxOption>
              </ListboxOptions>
            </transition>
          </div>
        </Listbox>
      </div>

      <!-- Comparison Scope -->
      <div class="space-y-2">
        <Text size="sm" weight="medium" class="text-gray-300">Comparison Scope</Text>
        <Listbox 
          :modelValue="props.strategy.comparison_scope"
          @update:modelValue="value => handleChange('comparison_scope', value)"
        >
          <div class="relative">
            <ListboxButton
              class="relative w-full cursor-pointer rounded-lg py-2 pl-3 pr-10 text-left bg-[#1A1D23] text-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <span class="block truncate">
                {{ comparisonScopes.find(s => s.id === props.strategy.comparison_scope)?.name }}
              </span>
              <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                <ChevronUpDownIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
              </span>
            </ListboxButton>

            <transition
              leave-active-class="transition duration-100 ease-in"
              leave-from-class="opacity-100"
              leave-to-class="opacity-0"
            >
              <ListboxOptions
                class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md py-1 bg-[#1A1D23] text-gray-300 focus:outline-none text-base sm:text-sm"
              >
                <ListboxOption
                  v-for="scope in comparisonScopes"
                  :key="scope.id"
                  :value="scope.id"
                  v-slot="{ active, selected }"
                >
                  <li
                    :class="[
                      'relative cursor-pointer select-none py-2 pl-10 pr-4',
                      active ? 'bg-[#1E2128]' : '',
                      selected ? 'bg-indigo-600 text-white' : ''
                    ]"
                  >
                    <span :class="['block truncate', selected ? 'font-medium' : 'font-normal']">
                      {{ scope.name }}
                    </span>
                    <span
                      v-if="selected"
                      class="absolute inset-y-0 left-0 flex items-center pl-3 text-white"
                    >
                      <CheckIcon class="h-5 w-5" aria-hidden="true" />
                    </span>
                  </li>
                </ListboxOption>
              </ListboxOptions>
            </transition>
          </div>
        </Listbox>
      </div>
    </div>
  </div>
</template>
