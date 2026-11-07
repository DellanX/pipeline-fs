import { useStages } from '@/composables/useStages';

export function useIODirectory() {
  const {
    stageSpaces,
    stagesLoading,
    stagesError,
    usagePercent,
    stageAvailableBytes,
    formatBytes,
  } = useStages();

  return {
    stageSpaces,
    stagesLoading,
    stagesError,
    usagePercent,
    stageAvailableBytes,
    formatBytes,
  };
}
