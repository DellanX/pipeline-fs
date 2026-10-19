<template>
  <div class="min-h-screen bg-gray-50 text-gray-900 transition-colors dark:bg-gray-950 dark:text-gray-100">
    <Navigation />

    <main class="p-4">
      <RouterView />
    </main>
  </div>
</template>

<script setup>
import Navigation from './Navigation.vue';
</script>