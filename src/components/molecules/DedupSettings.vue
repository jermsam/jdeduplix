<!-- DedupSettings.vue -->
<script setup lang="ts">
  import Slider from '../atoms/Slider.vue';
  import {DedupStrategy,  DEDUP_PRESETS, get_default_strategy_by_preset} from '../../types/dedup.ts';
  import {ref, watch} from 'vue';


  const props = defineProps<{
    strategy: DedupStrategy
    isDark?: boolean
  }>();

  const emit = defineEmits<{
    (e: 'update:strategy', value: DedupStrategy): void
  }>();


  const presets = DEDUP_PRESETS;
  const selectedPreset = ref<Preset>(get_default_strategy_by_preset('Exact Match'));

  watch(() => props.strategy, (newVal, oldVal) => {
    if (newVal && newVal !== oldVal) {
      selectedPreset.value = presets.find((preset) =>  JSON.stringify(preset.settings) === JSON.stringify(newVal))
    }
  }, {immediate: true, deep: true});


  function updateStrategy(key: keyof DedupStrategy, value: any) {
    emit('update:strategy', {
      ...props.strategy,
      [key]: value,
    });
  }

  function setPreset(preset: any) {
    emit('update:strategy', {...preset.settings});
  }
  watch(selectedPreset, setPreset)
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
            class="group relative p-4 text-left text-gray-600 dark:text-gray-100 bg-white/95 dark:bg-gray-900/95 rounded-xl ring-1 ring-black/[0.04] dark:ring-white/[0.04] hover:ring-black/[0.06] dark:hover:ring-white/[0.06] transition-all duration-300 ease-[cubic-bezier(.2,.8,.4,1)] transform hover:scale-[1.008] hover:-translate-y-[2px] shadow-[0_1px_3px_-1px_rgba(0,0,0,0.02),0_0_2px_0_rgba(99,102,241,0.05)] dark:shadow-[0_2px_4px_-2px_rgba(0,0,0,0.3),0_0_3px_0_rgba(129,140,248,0.15)] backdrop-blur-[8px] backdrop-saturate-[1.3] before:absolute before:inset-0 before:rounded-xl before:bg-gradient-to-br before:from-indigo-50/0 before:to-indigo-50/0 before:transition-colors before:duration-300 hover:before:from-indigo-50/50 hover:before:to-transparent dark:hover:before:from-indigo-500/[0.12] dark:hover:before:to-transparent before:pointer-events-none after:absolute after:inset-[1px] after:rounded-[10px] after:bg-gradient-to-br after:from-white/40 after:to-white/0 dark:after:from-white/[0.02] dark:after:to-white/0 after:pointer-events-none"
            :class="{
            'ring-1 ring-indigo-500/30 dark:ring-indigo-400/30 shadow-[0_0_0_1px_rgba(99,102,241,0.12),0_4px_8px_-4px_rgba(99,102,241,0.1)] dark:shadow-[0_0_0_1px_rgba(129,140,248,0.12),0_4px_8px_-4px_rgba(129,140,248,0.15)] bg-gradient-to-br from-white via-indigo-50/20 to-white dark:from-gray-900 dark:via-indigo-400/[0.08] dark:to-gray-900 scale-[1.008] -translate-y-[2px] before:from-indigo-50/40 before:to-transparent dark:before:from-indigo-400/15 dark:before:to-transparent text-gray-700 dark:text-white': selectedPreset === preset || (
              Math.abs(props.strategy.similarity_threshold - preset.settings.similarity_threshold) < 0.01 &&
              props.strategy.case_sensitive === preset.settings.case_sensitive &&
              props.strategy.ignore_whitespace === preset.settings.ignore_whitespace &&
              props.strategy.ignore_punctuation === preset.settings.ignore_punctuation
            ),
            'bg-gradient-to-br from-indigo-50 via-white to-indigo-50/40 dark:from-indigo-500/20 dark:via-gray-900 dark:to-indigo-500/10 ring-1 ring-indigo-500/40 dark:ring-indigo-400/30 scale-[1.012] -translate-y-[3px] before:from-indigo-100/60 before:to-transparent dark:before:from-indigo-400/[0.18] dark:before:to-transparent shadow-[0_8px_16px_-6px_rgba(99,102,241,0.15),0_2px_6px_-2px_rgba(99,102,241,0.1)] dark:shadow-[0_8px_16px_-6px_rgba(129,140,248,0.25),0_2px_6px_-2px_rgba(129,140,248,0.15)] text-gray-900 dark:text-white font-semibold': selectedPreset?.name === preset.name,
            'group-hover:shadow-[0_6px_16px_-6px_rgba(99,102,241,0.2),0_2px_8px_-2px_rgba(99,102,241,0.1)] dark:group-hover:shadow-[0_6px_16px_-6px_rgba(129,140,248,0.3),0_2px_8px_-2px_rgba(129,140,248,0.15)]': true,
            'after:absolute after:inset-[1px] after:rounded-[10px] after:ring-1 after:ring-inset after:ring-white/10 dark:after:ring-white/[0.03] after:pointer-events-none': true
          }"
            @click="selectedPreset = preset"
        >
          <div class="flex items-center justify-between mb-2">
            <div class="text-sm font-medium text-slate-800 dark:text-gray-100">{{ preset.name }}</div>
            <div class="px-2 py-0.5 text-xs font-medium rounded-full"
                 :class="{
                'bg-green-50 text-green-700 dark:bg-green-900/30 dark:text-green-400': preset.settings.similarity_threshold >= 0.95,
                'bg-yellow-50 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400': preset.settings.similarity_threshold >= 0.85 && preset.settings.similarity_threshold < 0.95,
                'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400': preset.settings.similarity_threshold < 0.85
              }"
            >
              {{ (preset.settings.similarity_threshold * 100).toFixed() }}%
            </div>
          </div>
          <div class="text-xs text-slate-500 dark:text-gray-400 mb-3">{{ preset.description }}</div>
          <div class="flex flex-wrap gap-1.5">
            <span v-if="preset.settings.case_sensitive"
                  class="px-1.5 py-0.5 text-[10px] font-medium bg-slate-100 dark:bg-gray-800 text-slate-600 dark:text-gray-400 rounded-full">Case Sensitive</span>
            <span v-if="!preset.settings.ignore_whitespace"
                  class="px-1.5 py-0.5 text-[10px] font-medium bg-slate-100 dark:bg-gray-800 text-slate-600 dark:text-gray-400 rounded-full">Exact Spacing</span>
            <span v-if="!preset.settings.ignore_punctuation"
                  class="px-1.5 py-0.5 text-[10px] font-medium bg-slate-100 dark:bg-gray-800 text-slate-600 dark:text-gray-400 rounded-full">Exact Punctuation</span>
          </div>
        </button>
      </div>
    </div>

    <!-- Advanced Settings -->
    <div
        class="space-y-4 bg-white dark:bg-gray-900 rounded-xl ring-1 ring-slate-200 dark:ring-gray-800 p-4 shadow-[0_2px_8px_-3px_rgba(0,0,0,0.05),0_2px_3px_-3px_rgba(0,0,0,0.05)] dark:shadow-[0_2px_8px_-3px_rgba(0,0,0,0.3),0_2px_3px_-3px_rgba(0,0,0,0.2)] [background-image:radial-gradient(rgba(0,0,0,0.015)_1px,transparent_1px)] dark:[background-image:radial-gradient(rgba(255,255,255,0.015)_1px,transparent_1px)] [background-size:16px_16px]">
      <div class="flex items-center justify-between">
        <div class="text-sm font-medium text-slate-800 dark:text-gray-100">Advanced Settings</div>
        <div class="text-xs text-slate-500 dark:text-gray-400">Fine-tune your matching preferences</div>
      </div>
      
      <div class="space-y-4">
        <div>
          <label class="text-sm font-medium text-slate-800 dark:text-gray-100 mb-2 block">
            Similarity Threshold: {{ (props.strategy.similarity_threshold * 100).toFixed() }}%
          </label>
          <Slider
              v-model="props.strategy.similarity_threshold"
              label="Similarity Threshold"
              @update:modelValue="updateStrategy('similarity_threshold', $event)"
          />
        </div>

        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <label class="text-sm text-slate-600 dark:text-gray-300">Case Sensitive</label>
            <button
                class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200"
                :class="props.strategy.case_sensitive ? 'bg-indigo-500 dark:bg-indigo-400' : 'bg-slate-200 dark:bg-gray-700'"
                @click="updateStrategy('case_sensitive', !props.strategy.case_sensitive)"
            >
              <span
                  class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200"
                  :class="props.strategy.case_sensitive ? 'translate-x-6' : 'translate-x-1'"
              />
            </button>
          </div>

          <div class="flex items-center justify-between">
            <label class="text-sm text-slate-600 dark:text-gray-300">Ignore Whitespace</label>
            <button
                class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200"
                :class="props.strategy.ignore_whitespace ? 'bg-indigo-500 dark:bg-indigo-400' : 'bg-slate-200 dark:bg-gray-700'"
                @click="updateStrategy('ignore_whitespace', !props.strategy.ignore_whitespace)"
            >
              <span
                  class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200"
                  :class="props.strategy.ignore_whitespace ? 'translate-x-6' : 'translate-x-1'"
              />
            </button>
          </div>

          <div class="flex items-center justify-between">
            <label class="text-sm text-slate-600 dark:text-gray-300">Ignore Punctuation</label>
            <button
                class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200"
                :class="props.strategy.ignore_punctuation ? 'bg-indigo-500 dark:bg-indigo-400' : 'bg-slate-200 dark:bg-gray-700'"
                @click="updateStrategy('ignore_punctuation', !props.strategy.ignore_punctuation)"
            >
              <span
                  class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200"
                  :class="props.strategy.ignore_punctuation ? 'translate-x-6' : 'translate-x-1'"
              />
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
</style>
