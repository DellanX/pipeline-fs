import { computed } from 'vue';
import { useStages } from '@/composables/useStages';

export function useDiskUsage() {
  const { stageSpaces, stagesLoading, stagesError, formatBytes } = useStages();

  const otherDiskBytes = computed(() => {
    const capacity = stageSpaces.value?.totalCapacity;
    const serviceUsage = stageSpaces.value?.directoryUsage.totalBytes;
    if (!capacity || serviceUsage == null) return null;
    return Math.max(0, capacity.usedBytes - serviceUsage);
  });

  const summaryServicePercent = computed(() => {
    const capacity = stageSpaces.value?.totalCapacity.totalBytes;
    const serviceUsage = stageSpaces.value?.directoryUsage.totalBytes;
    if (!capacity || serviceUsage == null) return 0;
    return Math.min(100, (serviceUsage / capacity) * 100);
  });

  const summaryOtherPercent = computed(() => {
    const capacity = stageSpaces.value?.totalCapacity.totalBytes;
    const otherUsage = otherDiskBytes.value;
    if (!capacity || otherUsage == null) return 0;
    return Math.min(100 - summaryServicePercent.value, (otherUsage / capacity) * 100);
  });

  return {
    stageSpaces,
    stagesLoading,
    stagesError,
    otherDiskBytes,
    summaryServicePercent,
    summaryOtherPercent,
    formatBytes,
  };
}
