<template>
  <div class="stage-browser">
    <h2>Stage {{ stageIndex }}: {{ stagePath }}</h2>
    <div v-if="loading">Loading files...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else>
      <div class="file-grid">
        <FileItemCard 
          v-for="file in files" 
          :key="file.path"
          :file="file"
          @file-clicked="handleFileClick"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import FileItemCard from './FileItemCard.vue';

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
    error.value = `Failed to load files: ${err.message}`;
  } finally {
    loading.value = false;
  }
};

const handleFileClick = (file) => {
  // Emit event to parent component
  emit('file-clicked', file);
};

const emit = defineEmits(['file-clicked']);

onMounted(() => {
  fetchFiles();
});
</script>

<style scoped>
.stage-browser {
  margin-bottom: 2rem;
}

.file-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
}
</style>