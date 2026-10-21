<template>
  <div class="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
    <div class="p-6">
      <h2 class="text-xl font-semibold mb-4">Configuration</h2>
      
      <div v-if="loading" class="text-center py-8">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-indigo-500"></div>
        <p class="mt-2">Loading configuration...</p>
      </div>
      
      <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
        <strong class="font-bold">Error! </strong>
        <span class="block">{{ error }}</span>
      </div>
      
      <div v-else-if="config && config.stages" class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Port</label>
          <input 
            v-model.number="config.port" 
            type="number" 
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Mount Point</label>
          <input 
            v-model="config.mount_point" 
            type="text" 
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
      </div>
      
      <div v-else-if="!loading && !error" class="text-center py-8">
        <p>No configuration data available</p>
      </div>
      
      <div v-if="config && config.stages" class="mt-4">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Stages</label>
        <div 
          v-for="(stage, index) in config.stages" 
          :key="index"
          class="flex items-center mb-2"
        >
          <input 
            v-model="config.stages[index]" 
            type="text" 
            class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          />
          <button 
            v-if="config.stages.length > 1" 
            @click="removeStage(index)"
            class="ml-2 p-2 text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300"
          >
            Remove
          </button>
        </div>
        <button 
          @click="addStage" 
          class="mt-2 px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-indigo-500 dark:hover:bg-indigo-600"
        >
          Add Stage
        </button>
      </div>
      
      <div class="mt-6 flex justify-end space-x-3">
        <button 
          @click="() => saveConfig()" 
          :disabled="saving"
          class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:opacity-50 dark:bg-green-500 dark:hover:bg-green-600"
        >
          {{ saving ? 'Saving...' : 'Save Configuration' }}
        </button>
        <button 
          @click="() => loadConfig()" 
          class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 dark:bg-gray-500 dark:hover:bg-gray-600"
        >
          Reload Configuration
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useFetch } from '@/composables/useFetch.js';
import { useCloned } from '@vueuse/core';

interface Config {
  stages: string[];
  port: number | null;
  mount_point: string | null;
}

// Use the existing composables for API calls
const { data, execute: loadConfig, isFetching: loading, error } = useFetch('/config', {
  immediate: true,
  refetch: false,
}).json<Config>();

// Use useFetch composable for PUT request
const { execute: saveConfig, isFetching: saving, error: saveError } = useFetch('/config',  {
  method: 'PUT',
  headers: {
    'Content-Type': 'application/json',
  },
}, {
  immediate: false,
  beforeFetch: (ctx) => {
    if (data.value) {
      ctx.options.body = JSON.stringify(data.value);
    }
    return ctx;
  },
});

// Create a reactive config that can be mutated properly using useCloned
const { cloned: config } = useCloned(data, { deep: true });

// Direct mutation functions for stages array
const addStage = () => {
  if (!data.value) return;
  // Use spread operator to create a new array and update the reference
  data.value = { ...data.value, stages: [...data.value.stages, ''] };
};

const removeStage = (index: number) => {
  if (!data.value || data.value.stages.length <= 1) return;
  // Create new array without the removed element
  const newStages = [...data.value.stages];
  newStages.splice(index, 1);
  data.value = { ...data.value, stages: newStages };
};

// Watch for changes in data to ensure config stays in sync
watch(data, (newData) => {
  if (newData && newData.stages) {
    // Ensure we're working with a proper array reference
    config.value = { ...newData };
  }
}, { immediate: true });

</script>