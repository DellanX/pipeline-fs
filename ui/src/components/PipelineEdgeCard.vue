<template>
  <section class="rounded-xl border border-dashed border-indigo-300 bg-white/80 p-3 dark:border-indigo-800 dark:bg-gray-900/70">
    <div class="flex flex-col items-center gap-3 sm:flex-row sm:justify-between">
      <div class="flex min-w-0 items-center gap-3">
        <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-indigo-100 text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300" aria-hidden="true">
          <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="1.8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14m-5-5 5 5-5 5" />
          </svg>
        </span>
        <div class="min-w-0">
          <p class="text-xs font-semibold uppercase tracking-wide text-indigo-700 dark:text-indigo-300">{{ edgeTitle }}</p>
          <p class="truncate text-sm text-gray-600 dark:text-gray-300">
            {{ sourceLabel }} <span class="px-1 text-gray-400" aria-hidden="true">→</span> {{ destinationLabel }}
          </p>
          <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400">{{ edgeDescription }}</p>
        </div>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <button
          type="button"
          @click="$emit('manual')"
          class="rounded-lg bg-indigo-600 px-3 py-1.5 text-xs font-semibold text-white hover:bg-indigo-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:bg-indigo-500 dark:hover:bg-indigo-600 dark:focus-visible:outline-indigo-400"
          :aria-label="`Manually migrate from ${sourceLabel} to ${destinationLabel}`"
        >
          Migrate manually
        </button>
        <button
          type="button"
          @click="$emit('reconfigure')"
          class="rounded-lg border border-gray-300 px-3 py-1.5 text-xs font-semibold text-gray-700 hover:border-indigo-400 hover:bg-indigo-50 hover:text-indigo-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:border-gray-700 dark:text-gray-300 dark:hover:border-indigo-700 dark:hover:bg-indigo-950 dark:hover:text-indigo-300 dark:focus-visible:outline-indigo-400"
          :aria-label="`Reconfigure output from ${sourceLabel} to ${destinationLabel}`"
        >
          Reconfigure
        </button>
        <button
          type="button"
          @click="$emit('insert')"
          class="rounded-lg bg-indigo-600 px-3 py-1.5 text-xs font-semibold text-white hover:bg-indigo-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:bg-indigo-500 dark:hover:bg-indigo-600 dark:focus-visible:outline-indigo-400"
          :aria-label="`Insert a stage between ${sourceLabel} and ${destinationLabel}`"
        >
          + Insert
        </button>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  sourceLabel: string;
  destinationLabel: string;
  mode: 'activity' | 'schedule';
}>();

defineEmits<{
  manual: [];
  insert: [];
  reconfigure: [];
}>();

const edgeTitle = computed(() => props.mode === 'activity' ? 'Activity-triggered promotion' : 'Scheduled snapshot promotion');
const edgeDescription = computed(() =>
  props.mode === 'activity'
    ? 'Move data when writes have settled or the filesystem signals a sync.'
    : 'Promote snapshots on a schedule; rules may target ranges of stages.',
);
</script>
