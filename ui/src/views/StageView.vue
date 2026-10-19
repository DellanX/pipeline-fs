<template>
  <div>
    <div class="mb-4 flex items-center justify-between gap-4">
      <h2 class="text-lg font-semibold">Files in Stage {{ stageId }}</h2>
      <button @click="goBack" class="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-md transition-colors text-white">
        Back to Pipeline
      </button>
    </div>
      <div v-if="loading" class="text-center py-8">Loading files...</div>
      <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded dark:border-red-800 dark:bg-red-950 dark:text-red-200">
        {{ error }}
      </div>
      <div v-else-if="!connected" class="text-center py-8 text-gray-500">
        Service is unreachable.
      </div>
      <div v-else-if="!canQuery('filesEndpoint')" class="text-center py-8 text-gray-500">
        File browsing is not available from this service.
      </div>
      <div v-else-if="files">
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <div 
            v-for="file in files" 
            :key="file.path"
            @click="openFile(file)"
            :class="[
              'bg-gray-100 p-4 rounded-lg cursor-pointer hover:bg-gray-200 transition-colors flex items-center gap-2 dark:bg-gray-900 dark:hover:bg-gray-800',
              { 'font-bold': file.isDirectory }
            ]"
          >
            <span class="text-xl">{{ file.isDirectory ? '📁' : '📄' }}</span>
            <div>
              <div class="font-medium">{{ file.name }}</div>
              <div v-if="!file.isDirectory" class="text-sm text-gray-600 dark:text-gray-400">{{ formatSize(file.size) }}</div>
            </div>
          </div>
        </div>
      </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useHealth } from '../composables/useHealth.js';

const route = useRoute();
const router = useRouter();

const stageId = route.params.id;
const files = ref([]);
const loading = ref(false);
const error = ref(null);
const apiUrl = import.meta.env.VITE_API_URL;
const { connected, canQuery } = useHealth();

const fetchFiles = async () => {
  if (!stageId) return;
  
  loading.value = true;
  error.value = null;
  
  try {
    if (!canQuery('filesEndpoint')) return;

    const response = await fetch(`${apiUrl}/stages/${stageId}/files`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    files.value = await response.json();
  } catch (err) {
    error.value = `Failed to load files: ${err.message}`;
  } finally {
    loading.value = false;
  }
};

const goBack = () => {
  router.push('/');
};

const openFile = (file) => {
  if (file.isDirectory) {
    // Handle directory navigation
    console.log('Navigate to directory:', file.path);
  } else {
    // Handle file opening
    console.log('Open file:', file.path);
  }
};

const formatSize = (size) => {
  if (size < 1024) return size + ' B';
  if (size < 1048576) return (size / 1024).toFixed(1) + ' KB';
  if (size < 1073741824) return (size / 1048576).toFixed(1) + ' MB';
  return (size / 1073741824).toFixed(1) + ' GB';
};

watch(() => canQuery('filesEndpoint'), (enabled) => {
  if (enabled) fetchFiles();
}, { immediate: true });
</script>