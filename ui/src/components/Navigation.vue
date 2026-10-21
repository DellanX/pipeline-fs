<template>
    <header class="bg-indigo-900 text-white p-4 flex justify-between items-center">
      <h1 class="text-xl font-bold">Pipeline-FS File Browser</h1>
      <div class="controls flex items-center gap-2">
        <RouterLink to="/config" class="px-3 py-2 rounded-md hover:bg-indigo-800 transition-colors">Config</RouterLink>
        <ConnectionStatusIndicator />
        <ThemeToggle />
      </div>
    </header>
</template>

<script setup lang="ts">
import ThemeToggle from './ThemeToggle.vue';
import ConnectionStatusIndicator from './ConnectionStatusIndicator.vue';
</script>