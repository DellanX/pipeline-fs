<template>
  <section class="rounded-xl border border-indigo-200 bg-indigo-50 p-4 dark:border-indigo-900 dark:bg-indigo-950/50">
    <h3 class="font-semibold">Disk capacity</h3>
    <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
      Total and used space across unique filesystems
    </p>
    <template v-if="stageSpaces">
      <p class="mt-3 text-sm">
        <strong>{{ formatBytes(stageSpaces.totalCapacity.usedBytes) }}</strong> used
        <span class="px-1 text-gray-400">/</span>
        {{ formatBytes(stageSpaces.totalCapacity.totalBytes) }}
      </p>
      <div
        class="mt-2 flex h-2.5 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700"
        role="img"
        :aria-label="`Disk usage: service ${Math.round(summaryServicePercent)}%, other ${Math.round(summaryOtherPercent)}%`"
      >
        <div class="h-full bg-indigo-600 dark:bg-indigo-400" :style="{ width: `${summaryServicePercent}%` }"></div>
        <div class="h-full bg-amber-500 dark:bg-amber-400" :style="{ width: `${summaryOtherPercent}%` }"></div>
      </div>
      <div class="mt-2 flex justify-between gap-3 text-xs text-gray-600 dark:text-gray-300">
        <span>Service {{ Math.round(summaryServicePercent) }}% ({{ formatBytes(stageSpaces.directoryUsage.totalBytes) }})</span>
        <span>Other {{ Math.round(summaryOtherPercent) }}% ({{ formatBytes(otherDiskBytes) }})</span>
      </div>
      <p class="mt-2 text-xs text-gray-600 dark:text-gray-300">
        {{ formatBytes(stageSpaces.totalCapacity.availableBytes) }} available
      </p>
    </template>
    <p v-else-if="stagesError" class="mt-3 text-sm text-red-700 dark:text-red-300" role="alert">
      Failed to load disk capacity: {{ stagesError }}
    </p>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ stagesLoading ? 'Calculating capacity…' : 'Capacity information unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import { useDiskUsage } from '@/composables/useDiskUsage';

const {
  stageSpaces,
  stagesLoading,
  stagesError,
  otherDiskBytes,
  summaryServicePercent,
  summaryOtherPercent,
  formatBytes,
} = useDiskUsage();
</script>