import { storeToRefs } from 'pinia';
import type { StageSpace } from '@/types/StageSpace';
import { usePipelineStore } from '@/stores/pipeline';

export function useStages() {
  const store = usePipelineStore();
  const {
    stageSpaces,
    stagesLoading,
    stagesError,
  } = storeToRefs(store);

  function stageSpaceFor(path: string) {
    return stageSpaces.value?.stages.find((stage) => stage.path === path);
  }

  function usagePercent(space?: StageSpace) {
    if (!space?.effectiveCapacityBytes) return 0;
    return Math.min(100, Math.round(((space.directoryBytes ?? 0) / space.effectiveCapacityBytes) * 100));
  }

  function stageAvailableBytes(space?: StageSpace) {
    if (space?.effectiveCapacityBytes == null || space.directoryBytes == null) return null;
    return Math.max(0, space.effectiveCapacityBytes - space.directoryBytes);
  }

  function formatBytes(bytes: number | null | undefined) {
    if (bytes == null) return '—';
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const unitIndex = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
    return `${(bytes / 1024 ** unitIndex).toFixed(unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
  }

  return {
    stageSpaces,
    stagesLoading,
    stagesError,
    loadStages: store.loadStages,
    stageSpaceFor,
    usagePercent,
    stageAvailableBytes,
    formatBytes,
  };
}
