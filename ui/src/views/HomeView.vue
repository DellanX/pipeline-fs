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
    const response = await fetch(`${apiUrl.value}/config`);
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