<template>
  <div>
    <div class="mb-4 flex justify-end">
      <button @click="refreshAll" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors text-white">
        Refresh All
      </button>
    </div>
    
    <ConfigCard />
    
  </div>
</template>

<script setup>
import ConfigCard from '../components/ConfigCard.vue';
</script>