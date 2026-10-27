<template>
  <dialog
    ref="dialog"
    class="m-auto max-h-[90vh] w-[min(42rem,calc(100%-2rem))] rounded-2xl border border-gray-200 bg-white p-0 text-gray-900 shadow-2xl backdrop:bg-gray-950/60 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100"
    aria-labelledby="stage-output-title"
    @click="closeOnBackdrop"
    @close="emit('close')"
  >
    <div class="flex items-start justify-between gap-4 border-b border-gray-200 p-5 dark:border-gray-800 sm:p-6">
      <div>
        <p class="text-xs font-semibold uppercase tracking-[0.16em] text-indigo-600 dark:text-indigo-400">{{ mode === 'activity' ? 'IO handoff' : 'Snapshot promotion' }}</p>
        <h2 id="stage-output-title" class="mt-1 text-xl font-semibold">Reconfigure {{ sourceLabel }}</h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Settings for data moving to {{ destinationLabel }}.
        </p>
      </div>
      <button
        type="button"
        @click="close"
        class="rounded-lg border border-gray-300 px-2.5 py-1.5 text-sm font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400"
        aria-label="Close output settings"
        autofocus
      >
        Close
      </button>
    </div>

    <div class="space-y-4 p-5 sm:p-6">
      <p class="rounded-lg border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-900 dark:border-amber-900 dark:bg-amber-950/50 dark:text-amber-200">
        Stage output settings are planned, but are not yet supported by the configuration API. No changes can be saved here yet.
      </p>

      <section class="rounded-xl border border-gray-200 p-4 dark:border-gray-700">
        <template v-if="mode === 'activity'">
          <h3 class="font-semibold">Activity triggers</h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Stage 0 data can be promoted when writes have been quiet for a period, such as five minutes, or when a FUSE sync call arrives.
          </p>
        </template>
        <template v-else>
          <h3 class="font-semibold">Snapshot schedule</h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Future rules can promote snapshots on daily or monthly schedules and target ranges of stages.
          </p>
        </template>
      </section>

      <section class="rounded-xl border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="font-semibold">Destination</h3>
        <p class="mt-1 break-all font-mono text-sm text-gray-600 dark:text-gray-300">{{ destinationPath || destinationLabel }}</p>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Data advances to the next configured stage; merges terminate at the last configured stage.
        </p>
      </section>

      <section class="rounded-xl border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="font-semibold">Transformers</h3>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Data transformation steps for this edge are not available yet.
        </p>
      </section>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from 'vue';

defineProps<{
  sourceLabel: string;
  destinationLabel: string;
  destinationPath: string;
  mode: 'activity' | 'schedule';
}>();

const dialog = ref<HTMLDialogElement | null>(null);
const emit = defineEmits<{
  close: [];
}>();

onMounted(() => dialog.value?.showModal());

onBeforeUnmount(() => {
  if (dialog.value?.open) dialog.value.close();
});

function close() {
  if (dialog.value?.open) dialog.value.close();
}

function closeOnBackdrop(event: MouseEvent) {
  if (event.target === dialog.value) close();
}
</script>
