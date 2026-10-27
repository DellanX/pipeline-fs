<template>
  <section class="rounded-xl border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800 col-span-2">
    <div class="flex flex-wrap items-start justify-between gap-2">
      <div>
        <h3 class="font-semibold">IO directory</h3>
        <p class="mt-1 break-all font-mono text-xs text-gray-500 dark:text-gray-400">
          {{ config.mount_point || 'No IO directory configured' }}
        </p>
      </div>
      <span v-if="ioDirectoryData.stageSpaces?.ioDirectory?.filesystemId != null" class="rounded-full bg-gray-200 px-2 py-1 text-xs text-gray-700 dark:bg-gray-700 dark:text-gray-200">
        Filesystem {{ ioDirectoryData.stageSpaces.ioDirectory.filesystemId }}
      </span>
    </div>
    <template v-if="ioDirectoryData.stageSpaces?.ioDirectory && !ioDirectoryData.stageSpaces.ioDirectory.error && !ioDirectoryData.stageSpaces.ioDirectory.directoryError">
      <p class="mt-3 text-sm">
        <strong>{{ ioDirectoryData.formatBytes(ioDirectoryData.stageSpaces.ioDirectory.directoryBytes) }}</strong>
        ({{ ioDirectoryData.stageSpaces.ioDirectory.fileCount }} files)
        <span class="px-1 text-gray-400">/</span>
        {{ ioDirectoryData.formatBytes(ioDirectoryData.stageSpaces.ioDirectory.effectiveCapacityBytes) }} usable
      </p>
      <div class="mt-2 h-2.5 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
        <div class="h-full rounded-full bg-violet-500" :style="{ width: `${ioDirectoryData.usagePercent(ioDirectoryData.stageSpaces.ioDirectory)}%` }"></div>
      </div>
      <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
        {{ ioDirectoryData.formatBytes(ioDirectoryData.stageAvailableBytes(ioDirectoryData.stageSpaces.ioDirectory)) }} available for this directory
      </p>
    </template>
    <p v-else-if="ioDirectoryData.stageSpaces?.ioDirectory?.error" class="mt-3 text-sm text-amber-700 dark:text-amber-300">
      {{ ioDirectoryData.stageSpaces.ioDirectory.error }}
    </p>
    <p v-else-if="ioDirectoryData.stageSpaces?.ioDirectory?.directoryError" class="mt-3 text-sm text-amber-700 dark:text-amber-300">
      {{ ioDirectoryData.stageSpaces.ioDirectory.directoryError }}
    </p>
    <p v-else-if="ioDirectoryData.stageSpaces?.ioError" class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ ioDirectoryData.stageSpaces.ioError }}
    </p>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ ioDirectoryData.stagesLoading ? 'Loading filesystem space…' : 'Filesystem space unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import type { Config, StagesResponse } from '@/types/config';

const props = defineProps<{
  ioDirectoryData: {
    stageSpaces: StagesResponse | null;
    stagesLoading: boolean;
    config: Config;
    usagePercent: (space: any) => number;
    stageAvailableBytes: (space: any) => number | null;
    formatBytes: (bytes: number | null | undefined) => string;
  };
}>();
</script>