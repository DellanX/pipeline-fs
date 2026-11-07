import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Config } from '@/types/Config';
import type { StagesResponse } from '@/types/StagesResponse';

const apiBaseUrl = `${import.meta.env.VITE_API_URL}/api/v1`;

async function fetchApi(path: string, init?: RequestInit) {
  const response = await fetch(`${apiBaseUrl}${path}`, init);
  if (!response.ok) {
    throw new Error(`Request failed with status ${response.status}${response.statusText ? `: ${response.statusText}` : ''}`);
  }
  return response;
}

function errorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

export const usePipelineStore = defineStore('pipeline', () => {
  const config = ref<Config | null>(null);
  const stageSpaces = ref<StagesResponse | null>(null);
  const configLoading = ref(false);
  const stagesLoading = ref(false);
  const saving = ref(false);
  const configError = ref<string | null>(null);
  const stagesError = ref<string | null>(null);
  const saveError = ref<string | null>(null);

  let configRequest: Promise<boolean> | null = null;
  let stagesRequest: Promise<boolean> | null = null;

  function loadConfig(force = false): Promise<boolean> {
    if (configRequest) return configRequest;
    if (config.value && !force) return Promise.resolve(true);

    configLoading.value = true;
    configError.value = null;
    configRequest = (async () => {
      try {
        const response = await fetchApi('/config');
        config.value = await response.json() as Config;
        return true;
      } catch (error) {
        configError.value = errorMessage(error);
        return false;
      } finally {
        configLoading.value = false;
        configRequest = null;
      }
    })();

    return configRequest;
  }

  function loadStages(force = false): Promise<boolean> {
    if (stagesRequest) {
      return force ? stagesRequest.then(() => loadStages(true)) : stagesRequest;
    }
    if (stageSpaces.value && !force) return Promise.resolve(true);

    stagesLoading.value = true;
    stagesError.value = null;
    stagesRequest = (async () => {
      try {
        const response = await fetchApi('/stages');
        stageSpaces.value = await response.json() as StagesResponse;
        return true;
      } catch (error) {
        stagesError.value = errorMessage(error);
        return false;
      } finally {
        stagesLoading.value = false;
        stagesRequest = null;
      }
    })();

    return stagesRequest;
  }

  async function saveConfig(nextConfig: Config): Promise<boolean> {
    saving.value = true;
    saveError.value = null;

    try {
      await fetchApi('/config', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(nextConfig),
      });
      config.value = { ...nextConfig, stages: [...nextConfig.stages] };
      return true;
    } catch (error) {
      saveError.value = errorMessage(error);
      return false;
    } finally {
      saving.value = false;
    }
  }

  return {
    config,
    stageSpaces,
    configLoading,
    stagesLoading,
    saving,
    configError,
    stagesError,
    saveError,
    loadConfig,
    loadStages,
    saveConfig,
  };
});
