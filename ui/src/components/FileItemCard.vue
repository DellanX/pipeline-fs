<template>
  <div 
    @click="handleClick"
    :class="[
      'bg-gray-100 p-4 rounded-lg cursor-pointer hover:bg-gray-200 transition-colors flex items-center gap-2 dark:bg-gray-900 dark:hover:bg-gray-800',
      { 'font-bold': file.isDirectory }
    ]"
  >
    <div class="text-xl">{{ file.isDirectory ? '📁' : '📄' }}</div>
    <div class="flex-grow">
      <div class="font-medium">{{ file.name }}</div>
      <div v-if="!file.isDirectory" class="text-sm text-gray-600 dark:text-gray-400">{{ formatSize(file.size) }}</div>
    </div>
  </div>
</template>

<script setup>
import { defineProps, defineEmits } from 'vue';

const props = defineProps({
  file: {
    type: Object,
    required: true
  }
});

const emit = defineEmits(['file-clicked']);

const formatSize = (bytes) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const handleClick = () => {
  emit('file-clicked', props.file);
};
</script>