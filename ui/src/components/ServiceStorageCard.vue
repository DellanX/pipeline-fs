<template>
  <section class="rounded-xl border border-indigo-200 bg-indigo-50 p-4 dark:border-indigo-900 dark:bg-indigo-950/50">
    <h3 class="font-semibold">Service storage</h3>
    <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
      Allocated file space in stage directories (IO view excluded); overlapping paths or hard links may count data more than once
    </p>
    <template v-if="stageSpaces">
      <p class="mt-3 text-sm">
        <strong>{{ formatBytes(stageSpaces.directoryUsage.totalBytes) }}</strong>
        used by this service
      </p>
      <p class="mt-2 text-xs text-gray-600 dark:text-gray-300">
        {{ formatBytes(otherDiskBytes) }} other disk usage
        <span class="px-1 text-gray-400">·</span>
        {{ stageSpaces.directoryUsage.fileCount }} files across
        {{ stageSpaces.directoryUsage.scannedDirectoryCount }} of
        {{ stageSpaces.directoryUsage.configuredDirectoryCount }} directories scanned
      </p>
    </template>
    <p v-else-if="stagesError" class="mt-3 text-sm text-red-700 dark:text-red-300" role="alert">
      Failed to load storage data: {{ stagesError }}
    </p>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ stagesLoading ? 'Calculating directory contents…' : 'Directory usage unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import { useServiceStorage } from '@/composables/useServiceStorage';
import { useDiskUsage } from '@/composables/useDiskUsage';

const { stageSpaces, stagesLoading, stagesError, formatBytes } = useServiceStorage();
const { otherDiskBytes } = useDiskUsage();
</script>