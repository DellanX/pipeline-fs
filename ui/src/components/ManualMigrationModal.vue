<template>
  <dialog
    ref="dialog"
    class="m-auto max-h-[90vh] w-[min(44rem,calc(100%-2rem))] overflow-y-auto rounded-2xl border border-gray-200 bg-white p-0 text-gray-900 shadow-2xl backdrop:bg-gray-950/60 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100"
    aria-labelledby="manual-migration-title"
    @click="closeOnBackdrop"
    @close="emit('close')"
  >
    <div class="flex items-start justify-between gap-4 border-b border-gray-200 p-5 dark:border-gray-800 sm:p-6">
      <div>
        <p class="text-xs font-semibold uppercase tracking-[0.16em] text-indigo-600 dark:text-indigo-400">Manual migration</p>
        <h2 id="manual-migration-title" class="mt-1 text-xl font-semibold">Move data from {{ stageLabel(startIndex) }}</h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Choose how data advances through the configured stages.
        </p>
      </div>
      <button
        type="button"
        @click="close"
        class="rounded-lg border border-gray-300 px-2.5 py-1.5 text-sm font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400"
        aria-label="Close manual migration"
        autofocus
      >
        Close
      </button>
    </div>

    <div class="space-y-4 p-5 sm:p-6">
      <p class="rounded-lg border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-900 dark:border-amber-900 dark:bg-amber-950/50 dark:text-amber-200">
        Migration execution is not available in the service API yet. You can review the migration plan, but cannot run it.
      </p>

      <ol class="space-y-3">
        <li
          v-for="sourceIndex in visibleRows"
          :key="sourceIndex"
          class="rounded-xl border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-950"
        >
          <div class="mb-3 flex flex-wrap items-start justify-between gap-2">
            <div>
              <p class="text-xs font-semibold uppercase tracking-wide text-indigo-600 dark:text-indigo-400">
                {{ sourceIndex === lastIndex ? 'Final stage' : `Step ${sourceIndex - startIndex + 1}` }}
              </p>
              <p class="mt-1 text-sm font-medium">
                {{ stageLabel(sourceIndex) }}
                <template v-if="sourceIndex < lastIndex">
                  <span class="px-1 text-gray-400" aria-hidden="true">→</span>
                  {{ stageLabel(sourceIndex + 1) }} <span class="font-normal text-gray-500">({{ stagePaths[sourceIndex + 1] }})</span>
                </template>
              </p>
            </div>
            <span class="rounded-full px-2.5 py-1 text-xs font-medium" :class="occupancyBadgeClass(sourceIndex)">
              {{ occupancyLabel(sourceIndex) }}
            </span>
          </div>

          <div class="flex flex-wrap gap-2">
            <button
              v-for="option in optionsFor(sourceIndex)"
              :key="option.value"
              type="button"
              :aria-pressed="actions[sourceIndex] === option.value"
              :class="[
                'rounded-lg border px-3 py-2 text-sm font-semibold focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:focus-visible:outline-indigo-400',
                actions[sourceIndex] === option.value
                  ? 'border-indigo-600 bg-indigo-600 text-white dark:border-indigo-500 dark:bg-indigo-500'
                  : 'border-gray-300 bg-white text-gray-700 hover:border-indigo-400 hover:bg-indigo-50 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-300 dark:hover:border-indigo-700 dark:hover:bg-indigo-950',
              ]"
              @click="selectAction(sourceIndex, option.value)"
            >
              {{ option.label }}
            </button>
          </div>
          <p v-if="sourceIndex === lastIndex" class="mt-2 text-xs text-gray-500 dark:text-gray-400">
            This is the last configured stage; merge is the only available operation.
          </p>
          <p v-else-if="isOccupancyUnknown(sourceIndex)" class="mt-2 text-xs text-gray-500 dark:text-gray-400">
            Destination contents could not be determined, so it is treated as occupied.
          </p>
        </li>
      </ol>

      <p v-if="planSummary" class="rounded-lg border border-indigo-200 bg-indigo-50 px-4 py-3 text-sm text-indigo-950 dark:border-indigo-900 dark:bg-indigo-950/50 dark:text-indigo-100">
        <strong>Plan:</strong> {{ planSummary }}
      </p>
      <p v-else class="text-sm text-gray-500 dark:text-gray-400">
        Select an operation to build a migration plan.
      </p>
    </div>

    <div class="flex justify-end gap-3 border-t border-gray-200 p-5 dark:border-gray-800 sm:px-6">
      <button
        type="button"
        @click="close"
        class="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400"
      >
        Cancel
      </button>
      <button
        type="button"
        disabled
        class="cursor-not-allowed rounded-lg bg-indigo-600 px-4 py-2 text-sm font-semibold text-white opacity-50 dark:bg-indigo-500"
        :title="canMigrate ? 'Migration execution is not implemented yet' : 'The service does not support manual migration yet'"
      >
        Run migration
      </button>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue';
import type { StageSpace } from '@/types/StageSpace';

type MigrationAction = 'move' | 'push' | 'merge';
type MigrationOption = { value: MigrationAction; label: string };

const props = defineProps<{
  startIndex: number;
  stagePaths: string[];
  stageLabels: string[];
  stageSpaces: StageSpace[];
  canMigrate: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const dialog = ref<HTMLDialogElement | null>(null);
const actions = reactive<Record<number, MigrationAction>>({});
const lastIndex = computed(() => props.stagePaths.length - 1);
const visibleRows = computed(() => {
  const rows = [props.startIndex];
  while (rows[rows.length - 1] < lastIndex.value && actions[rows[rows.length - 1]] === 'push') {
    rows.push(rows[rows.length - 1] + 1);
  }
  return rows;
});

const planSummary = computed(() =>
  visibleRows.value
    .filter((index) => actions[index])
    .map((index) => {
      if (index === lastIndex.value) return `${actions[index]} at ${stageLabel(index)}`;
      return `${actions[index]} ${stageLabel(index)} → ${stageLabel(index + 1)}`;
    })
    .join('; '),
);

onMounted(() => dialog.value?.showModal());

onBeforeUnmount(() => {
  if (dialog.value?.open) dialog.value.close();
});

function stageLabel(index: number) {
  return props.stageLabels[index] ?? `Stage ${index}`;
}

function destinationSpace(sourceIndex: number) {
  const path = props.stagePaths[sourceIndex + 1];
  return props.stageSpaces.find((space) => space.path === path);
}

function isDestinationEmpty(sourceIndex: number) {
  const space = destinationSpace(sourceIndex);
  return space?.fileCount === 0 || (space?.fileCount == null && space?.directoryBytes === 0);
}

function isOccupancyUnknown(sourceIndex: number) {
  if (sourceIndex === lastIndex.value) return false;
  const space = destinationSpace(sourceIndex);
  return !space || (space.fileCount == null && space.directoryBytes == null);
}

function occupancyLabel(sourceIndex: number) {
  if (sourceIndex === lastIndex.value) return 'Final stage';
  if (isDestinationEmpty(sourceIndex)) return 'Empty';
  if (isOccupancyUnknown(sourceIndex)) return 'Occupancy unknown';
  return 'Contains data';
}

function occupancyBadgeClass(sourceIndex: number) {
  return isDestinationEmpty(sourceIndex)
    ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300'
    : 'bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300';
}

function optionsFor(sourceIndex: number): MigrationOption[] {
  if (sourceIndex === lastIndex.value) return [{ value: 'merge', label: 'Merge' }];
  if (isDestinationEmpty(sourceIndex)) return [{ value: 'move', label: 'Move' }];
  if (sourceIndex + 1 === lastIndex.value) return [{ value: 'merge', label: 'Merge' }];
  return [
    { value: 'push', label: 'Push' },
    { value: 'merge', label: 'Merge' },
  ];
}

function selectAction(sourceIndex: number, action: MigrationAction) {
  actions[sourceIndex] = action;
  for (const index of Object.keys(actions).map(Number)) {
    if (index > sourceIndex) delete actions[index];
  }
}

function close() {
  if (dialog.value?.open) dialog.value.close();
}

function closeOnBackdrop(event: MouseEvent) {
  if (event.target === dialog.value) close();
}
</script>
