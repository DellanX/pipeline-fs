<template>
  <div class="min-h-screen bg-gray-50 text-gray-900 transition-colors dark:bg-gray-950 dark:text-gray-100">
    <header class="bg-indigo-900 text-white p-4 flex justify-between items-center">
      <h1 class="text-xl font-bold">Pipeline-FS File Browser</h1>
      <div class="controls flex items-center gap-2">
        <ThemeToggle />
        <button @click="refreshAll" class="ml-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors">
          Refresh All
        </button>
      </div>
    </header>
    
    <main class="p-4">
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
      <div v-else-if="!loading && !error">
        <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded dark:border-yellow-800 dark:bg-yellow-950 dark:text-yellow-200 mb-4">
          <p>Service is currently offline or unreachable.</p>
          <p class="mt-2">Please start the Rust service with:</p>
          <pre class="bg-gray-800 text-white p-2 rounded mt-2 text-sm">./target/debug/pipeline-fs --mount-point /tmp/test-mount --stages /tmp/stage0 /tmp/stage1 --port 8080</pre>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import StageBrowser from '../components/StageBrowser.vue';
import ThemeToggle from '../components/ThemeToggle.vue';

const config = ref(null);
const loading = ref(false);
const error = ref(null);
const apiUrl = ref('/api');

const fetchPipelineConfig = async () => {
  loading.value = true;
  error.value = null;
  
  try {
    // First check if service is reachable
    const healthResponse = await fetch(`${apiUrl.value}/health`);
    if (!healthResponse.ok) {
      throw new Error(`Service not reachable: ${healthResponse.status}`);
    }
    
    const healthData = await healthResponse.json();
    if (!healthData.is_healthy) {
      throw new Error('Service is unhealthy');
    }
    
    // Only fetch config if service is healthy
    const response = await fetch(`${apiUrl.value}/config`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    config.value = await response.json();
  } catch (err) {
    // If we can't reach the service or it's unhealthy, don't show an error
    // but let the user know in a friendly way
    if (err.message.includes('Service not reachable') || err.message.includes('Service is unhealthy')) {
      config.value = null; // Clear any previous config
    } else {
      error.value = `Failed to load pipeline configuration: ${err.message}`;
    }
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