<template>
  <div class="border border-gray-300 rounded-lg p-4 mb-6 dark:border-gray-700">
    <h2 class="text-lg font-semibold mb-3">Stage {{ stageIndex }}: {{ stagePath }}</h2>
    <div v-if="loading" class="text-center py-4">Loading files...</div>
    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded dark:border-red-800 dark:bg-red-950 dark:text-red-200">
      {{ error }}
    </div>
    <div v-else-if="!files.length && !loading" class="text-center py-4 text-gray-500 dark:text-gray-400">
      No files found in this stage.
    </div>
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <div 
        v-for="file in files" 
        :key="file.path"
        @click="openFile(file)"
        :class="[
          'flex items-center p-3 cursor-pointer border border-gray-200 rounded hover:bg-gray-50 transition-colors dark:border-gray-700 dark:hover:bg-gray-800',
          { 'font-bold': file.isDirectory }
        ]">
        <span class="mr-2 w-6 text-center">{{ file.isDirectory ? '📁' : '📄' }}</span>
        <div class="flex-grow">
          <div class="font-medium">{{ file.name }}</div>
          <div v-if="!file.isDirectory" class="text-sm text-gray-600 dark:text-gray-400">{{ formatSize(file.size) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const props = defineProps({
  stageIndex: Number,
  stagePath: String,
  apiUrl: String
});

const files = ref([]);
const loading = ref(false);
const error = ref(null);

const fetchFiles = async () => {
  if (!props.stagePath) return;
  
  loading.value = true;
  error.value = null;
  
  try {
    const response = await fetch(`${props.apiUrl}/stages/${props.stageIndex}/files`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    files.value = await response.json();
  } catch (err) {
    // If we get a network error, it's likely the service is offline
    if (err.message.includes('Failed to fetch')) {
      error.value = 'Service is currently offline. Please start the Rust service.';
    } else {
      error.value = `Failed to load files: ${err.message}`;
    }
  } finally {
    loading.value = false;
  }
};

const openFile = (file) => {
  if (!file.isDirectory) {
    // Handle file opening
    console.log('Opening file:', file.path);
  }
};

const formatSize = (bytes) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

onMounted(() => {
  fetchFiles();
});
</script>