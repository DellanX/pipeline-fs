<template>
  <div 
    class="flex items-center cursor-pointer"
    @click="refreshStatus"
    :title="statusText"
  >
    <div :class="statusClass" class="w-3 h-3 rounded-full mr-2"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHealth } from '@/composables/useHealth.js';

const { status, refreshStatus } = useHealth();

const statusClass = computed(() => {
  if (status.value.isHealthy) {
    return 'bg-green-500';
  } else if (status.value.connected) {
    return 'bg-red-500';
  }
  return 'bg-gray-500';
});

const statusText = computed(() => {
  if (status.value.isHealthy) {
    return 'Service is healthy';
  } else if (status.value.connected) {
    return 'Service is unhealthy';
  }
  return 'Service is unreachable';
});
</script>