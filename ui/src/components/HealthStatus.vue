<template>
  <div class="bg-gray-100 p-4 rounded-lg mb-6 dark:bg-gray-900">
    <h2 class="text-lg font-semibold mb-2">Service Status</h2>
    <div class="flex items-center">
      <div :class="statusClass" class="w-3 h-3 rounded-full mr-2"></div>
      <span>{{ statusText }}</span>
      <span v-if="lastChecked" class="ml-4 text-sm text-gray-500 dark:text-gray-400">Last checked: {{ lastChecked }}</span>
    </div>
    <div v-if="healthData" class="mt-2 text-sm">
      <p>Version: {{ healthData.version }}</p>
      <p>Timestamp: {{ new Date(healthData.timestamp * 1000).toLocaleString() }}</p>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, watch } from 'vue';
import { useServiceHealth } from '@/composables/useServiceHealth';

const props = defineProps({
  apiUrl: {
    type: String,
    required: true
  }
});

const { status, refreshStatus } = useServiceHealth(props.apiUrl);

const healthData = computed(() => status.value.healthData);
const lastChecked = computed(() => status.value.lastChecked ? status.value.lastChecked.toLocaleTimeString() : null);

const statusClass = computed(() => {
  if (status.value.isHealthy) {
    return 'bg-green-500';
  } else if (status.value.isUnhealthy) {
    return 'bg-red-500';
  }
  return 'bg-gray-500';
});

const statusText = computed(() => {
  if (status.value.isHealthy) {
    return 'Service is healthy';
  } else if (status.value.isUnhealthy) {
    return 'Service is unhealthy';
  }
  return 'Status unknown';
});

// Watch for API URL changes
watch(() => props.apiUrl, () => {
  refreshStatus();
});
</script>