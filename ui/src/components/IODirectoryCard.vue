<template>
  <section class="rounded-xl border border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800 col-span-2">
    <div class="flex flex-wrap items-start justify-between gap-2">
      <div>
        <h3 class="font-semibold">IO directory</h3>
        <p class="mt-1 break-all font-mono text-xs text-gray-500 dark:text-gray-400">
          {{ config.mount_point || 'No IO directory configured' }}
        </p>
      </div>
      <span v-if="stageSpaces?.ioDirectory?.filesystemId != null" class="rounded-full bg-gray-200 px-2 py-1 text-xs text-gray-700 dark:bg-gray-700 dark:text-gray-200">
        Filesystem {{ stageSpaces.ioDirectory.filesystemId }}
      </span>
    </div>
    <template v-if="stageSpaces?.ioDirectory && !stageSpaces.ioDirectory.error && !stageSpaces.ioDirectory.directoryError">
      <p class="mt-3 text-sm">
        <strong>{{ formatBytes(stageSpaces.ioDirectory.directoryBytes) }}</strong>
        ({{ stageSpaces.ioDirectory.fileCount }} files)
        <span class="px-1 text-gray-400">/</span>
        {{ formatBytes(stageSpaces.ioDirectory.effectiveCapacityBytes) }} usable
      </p>
      <div class="mt-2 h-2.5 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
        <div class="h-full rounded-full bg-violet-500" :style="{ width: `${usagePercent(stageSpaces.ioDirectory)}%` }"></div>
      </div>
      <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
        {{ formatBytes(stageAvailableBytes(stageSpaces.ioDirectory)) }} available for this directory
      </p>
    </template>
    <p v-else-if="stageSpaces?.ioDirectory?.error" class="mt-3 text-sm text-amber-700 dark:text-amber-300">
      {{ stageSpaces.ioDirectory.error }}
    </p>
    <p v-else-if="stageSpaces?.ioDirectory?.directoryError" class="mt-3 text-sm text-amber-700 dark:text-amber-300">
      {{ stageSpaces.ioDirectory.directoryError }}
    </p>
    <p v-else-if="stageSpaces?.ioError" class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ stageSpaces.ioError }}
    </p>
    <p v-else-if="stagesError" class="mt-3 text-sm text-red-700 dark:text-red-300" role="alert">
      Failed to load IO storage data: {{ stagesError }}
    </p>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ stagesLoading ? 'Loading filesystem space…' : 'Filesystem space unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import type { Config } from '@/types/Config';
import { useIODirectory } from '@/composables/useIODirectory';

defineProps<{ config: Config }>();

const {
  stageSpaces,
  stagesLoading,
  stagesError,
  usagePercent,
  stageAvailableBytes,
  formatBytes,
} = useIODirectory();
</script>