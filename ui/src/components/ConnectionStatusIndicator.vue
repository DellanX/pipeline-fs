<template>
  <div 
    class="flex items-center cursor-pointer"
    @click="refreshStatus"
    :title="statusText"
  >
    <div :class="statusClass" class="w-3 h-3 rounded-full mr-2"></div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useServiceHealth } from '../composables/useServiceHealth';

const props = defineProps({
  apiUrl: {
    type: String,
    required: true
  }
});

const { status, refreshStatus } = useServiceHealth(props.apiUrl);

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
</script>