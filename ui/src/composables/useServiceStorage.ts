import { useStages } from '@/composables/useStages';

export function useServiceStorage() {
  const { stageSpaces, stagesLoading, stagesError, formatBytes } = useStages();
  return { stageSpaces, stagesLoading, stagesError, formatBytes };
}
