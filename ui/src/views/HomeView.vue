<template>
  <div>
    <div class="mb-4 flex justify-end">
      <button @click="refreshAll" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors text-white">
        Refresh All
      </button>
    </div>
      <div v-if="loading" class="text-center py-8">Loading pipeline configuration...</div>
      <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded dark:border-red-800 dark:bg-red-950 dark:text-red-200">
        {{ error }}
      </div>
      <div v-else-if="config">
        <div class="bg-gray-100 p-4 rounded-lg mb-6 dark:bg-gray-900">
          <h2 class="text-lg font-semibold mb-2">Pipeline Configuration</h2>
          <p>Mount Point: {{ config.mount_point }}</p>
          <p>Stages: {{ config.stages.length }}</p>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <StageBrowser 
            v-for="(stage, index) in config.stages" 
            :key="index"
            :stage-index="index"
            :stage-path="stage.path"
            :api-url="apiUrl"
          />
        </div>
      </div>
      <div v-else-if="!loading && !error && connected">
        <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded dark:border-yellow-800 dark:bg-yellow-950 dark:text-yellow-200 mb-4">
          The service is connected, but does not advertise a configuration endpoint.
        </div>
      </div>
      <div v-else-if="!loading && !error">
        <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded dark:border-yellow-800 dark:bg-yellow-950 dark:text-yellow-200 mb-4">
          <p>Service is currently offline or unreachable.</p>
          <p class="mt-2">Please start the Rust service with:</p>
          <pre class="bg-gray-800 text-white p-2 rounded mt-2 text-sm">./target/debug/pipeline-fs --mount-point /tmp/test-mount --stages /tmp/stage0 /tmp/stage1 --port 8080</pre>
        </div>
      </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import StageBrowser from '../components/StageBrowser.vue';
import { useHealth } from '../composables/useHealth.js';

const config = ref(null);
const loading = ref(false);
const error = ref(null);
const apiUrl = import.meta.env.VITE_API_URL;
const { connected, canQuery, refreshStatus } = useHealth();

const fetchPipelineConfig = async () => {
  loading.value = true;
  error.value = null;
  
  try {
    await refreshStatus();
    if (!canQuery('configEndpoint')) {
      config.value = null;
      return;
    }

    const response = await fetch(`${apiUrl}/config`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    config.value = await response.json();
  } catch (err) {
    error.value = `Failed to load pipeline configuration: ${err.message}`;
  } finally {
    loading.value = false;
  }
};

const refreshAll = () => {
  fetchPipelineConfig();
};

onMounted(() => {
  fetchPipelineConfig();
});
</script>