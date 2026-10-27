<template>
  <section class="rounded-xl border border-indigo-200 bg-indigo-50 p-4 dark:border-indigo-900 dark:bg-indigo-950/50">
    <h3 class="font-semibold">Service storage</h3>
    <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
      Allocated file space in stage directories (IO view excluded); overlapping paths or hard links may count data more than once
    </p>
    <template v-if="serviceStorageData.stageSpaces">
      <p class="mt-3 text-sm">
        <strong>{{ serviceStorageData.formatBytes(serviceStorageData.stageSpaces.directoryUsage.totalBytes) }}</strong>
        used by this service
      </p>
      <p class="mt-2 text-xs text-gray-600 dark:text-gray-300">
        {{ serviceStorageData.formatBytes(serviceStorageData.otherDiskBytes) }} other disk usage
        <span class="px-1 text-gray-400">·</span>
        {{ serviceStorageData.stageSpaces.directoryUsage.fileCount }} files across
        {{ serviceStorageData.stageSpaces.directoryUsage.scannedDirectoryCount }} of
        {{ serviceStorageData.stageSpaces.directoryUsage.configuredDirectoryCount }} directories scanned
      </p>
    </template>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ serviceStorageData.stagesLoading ? 'Calculating directory contents…' : 'Directory usage unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import type { StagesResponse } from '@/types/config';

const props = defineProps<{
  serviceStorageData: {
    stageSpaces: StagesResponse | null;
    stagesLoading: boolean;
    otherDiskBytes: number | null | undefined;
    formatBytes: (bytes: number | null | undefined) => string;
  };
}>();
</script>