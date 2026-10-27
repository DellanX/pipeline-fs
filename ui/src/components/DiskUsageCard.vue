<template>
  <section class="rounded-xl border border-indigo-200 bg-indigo-50 p-4 dark:border-indigo-900 dark:bg-indigo-950/50">
    <h3 class="font-semibold">Disk capacity</h3>
    <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
      Total and used space across unique filesystems
    </p>
    <template v-if="diskUsageData.stageSpaces">
      <p class="mt-3 text-sm">
        <strong>{{ diskUsageData.formatBytes(diskUsageData.stageSpaces.totalCapacity.usedBytes) }}</strong> used
        <span class="px-1 text-gray-400">/</span>
        {{ diskUsageData.formatBytes(diskUsageData.stageSpaces.totalCapacity.totalBytes) }}
      </p>
      <div
        class="mt-2 flex h-2.5 overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700"
        role="img"
        :aria-label="`Disk usage: service ${Math.round(diskUsageData.summaryServicePercent)}%, other ${Math.round(diskUsageData.summaryOtherPercent)}%`"
      >
        <div class="h-full bg-indigo-600 dark:bg-indigo-400" :style="{ width: `${diskUsageData.summaryServicePercent}%` }"></div>
        <div class="h-full bg-amber-500 dark:bg-amber-400" :style="{ width: `${diskUsageData.summaryOtherPercent}%` }"></div>
      </div>
      <div class="mt-2 flex justify-between gap-3 text-xs text-gray-600 dark:text-gray-300">
        <span>Service {{ Math.round(diskUsageData.summaryServicePercent) }}% ({{ diskUsageData.formatBytes(diskUsageData.stageSpaces.directoryUsage.totalBytes) }})</span>
        <span>Other {{ Math.round(diskUsageData.summaryOtherPercent) }}% ({{ diskUsageData.formatBytes(diskUsageData.otherDiskBytes) }})</span>
      </div>
      <p class="mt-2 text-xs text-gray-600 dark:text-gray-300">
        {{ diskUsageData.formatBytes(diskUsageData.stageSpaces.totalCapacity.availableBytes) }} available
      </p>
    </template>
    <p v-else class="mt-3 text-sm text-gray-500 dark:text-gray-400">
      {{ diskUsageData.stagesLoading ? 'Calculating capacity…' : 'Capacity information unavailable.' }}
    </p>
  </section>
</template>

<script setup lang="ts">
import { StagesResponse } from '@/types';

const props = defineProps<{
  diskUsageData: {
    stageSpaces: StagesResponse | null;
    stagesLoading: boolean;
    otherDiskBytes: any;
    summaryServicePercent: number;
    summaryOtherPercent: number;
    formatBytes: (bytes: number | null | undefined) => string;
  };
}>();
</script>