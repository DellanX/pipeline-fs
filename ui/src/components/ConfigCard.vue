<template>
  <section class="overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm dark:border-gray-800 dark:bg-gray-900">
    <header class="border-b border-gray-200 px-6 py-5 dark:border-gray-800">
      <div class="flex flex-wrap items-start justify-between gap-4">
        <div>
          <p class="text-sm font-semibold uppercase tracking-[0.16em] text-indigo-600 dark:text-indigo-400">Pipeline</p>
          <h2 class="mt-1 text-2xl font-semibold">Stage configuration</h2>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">New writes land in the IO stage, then progress through later storage tiers.</p>
        </div>
        <button
          type="button"
          @click="store.loadConfig(true)"
          :disabled="loading"
          class="rounded-lg border border-gray-300 px-3 py-2 text-sm font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 disabled:opacity-50 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400"
        >
          {{ loading ? 'Refreshing…' : 'Reload' }}
        </button>
      </div>
    </header>

    <div class="p-6">
      <div v-if="loading && !config" class="py-10 text-center text-gray-500 dark:text-gray-400">
        Loading configuration…
      </div>
      <div v-else-if="error" class="mb-5 rounded-lg border border-red-300 bg-red-50 px-4 py-3 text-sm text-red-800 dark:border-red-900 dark:bg-red-950 dark:text-red-200" role="alert">
        Failed to load configuration: {{ error }}
      </div>
      <template v-else-if="config">
        <div class="mb-5 grid grid-cols-1 gap-4 lg:grid-cols-2">
          <DiskUsageCard />

          <ServiceStorageCard />
        </div>

        <IODirectoryCard :config="config" />
        <p v-if="stageSpaceError && stageSpaces" class="mt-3 rounded-lg border border-amber-300 bg-amber-50 px-4 py-3 text-sm text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-200" role="status">
          Could not refresh storage details: {{ stageSpaceError }}
        </p>

        <section class="mt-6 rounded-2xl border border-indigo-100 bg-gradient-to-b from-indigo-50/80 to-white p-4 sm:p-6 dark:border-indigo-950 dark:from-indigo-950/40 dark:to-gray-900">
          <div class="mb-5 flex flex-wrap items-center justify-between gap-3">
            <div>
              <p class="text-xs font-semibold uppercase tracking-[0.16em] text-indigo-600 dark:text-indigo-400">Storage flow</p>
              <h3 class="mt-1 text-lg font-semibold">Pipeline stages</h3>
            </div>
            <span class="rounded-full border border-indigo-200 bg-white px-3 py-1 text-xs font-semibold text-indigo-700 dark:border-indigo-900 dark:bg-gray-900 dark:text-indigo-300">
              {{ config.stages.length }} {{ config.stages.length === 1 ? 'stage' : 'stages' }}
            </span>
          </div>

        <div v-if="!config.stages.length" class="flex flex-col items-center py-3">
          <span class="h-5 border-l-2 border-dashed border-indigo-300 dark:border-indigo-800" aria-hidden="true"></span>
          <button
            type="button"
            @click="insertStageAt(0)"
            class="cursor-pointer rounded-full border border-dashed border-indigo-400 px-4 py-2 text-sm font-medium text-indigo-700 transition-all hover:-translate-y-0.5 hover:border-indigo-600 hover:bg-indigo-100 hover:shadow-md focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 active:translate-y-0 dark:border-indigo-700 dark:text-indigo-300 dark:hover:border-indigo-500 dark:hover:bg-indigo-900 dark:focus-visible:outline-indigo-400"
          >
            ＋ Add first stage
          </button>
        </div>

        <div v-if="config.stages.length" class="my-4 flex flex-wrap items-center gap-3 rounded-xl border border-emerald-200 bg-emerald-50/70 px-4 py-3 dark:border-emerald-900 dark:bg-emerald-950/30">
          <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-emerald-100 text-emerald-700 dark:bg-emerald-900 dark:text-emerald-300" aria-hidden="true">→</span>
          <p class="text-sm text-emerald-950 dark:text-emerald-100">
            <strong>Aggregated directory writes directly to {{ stageLabel(0) }}.</strong>
            Changes are immediately available in the IO stage; this is not a migration edge.
          </p>
        </div>

        <!-- Display IO/Inlet stage (index 0) as a special visual element -->
        <div v-if="config.stages.length > 0" class="my-4 rounded-xl border border-emerald-200 bg-white p-4 shadow-sm dark:border-emerald-900 dark:bg-gray-900">
          <div class="flex flex-wrap items-start gap-3">
            <span class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-emerald-100 font-bold text-emerald-700 dark:bg-emerald-950 dark:text-emerald-300" aria-hidden="true">00</span>
            <div class="min-w-0 flex-1">
              <div class="mb-2 flex flex-wrap items-center gap-2">
                <h3 class="font-semibold">{{ stageLabel(0) }}</h3>
                <span class="rounded-full bg-emerald-50 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-emerald-700 dark:bg-emerald-950 dark:text-emerald-300">{{ config.stages.length === 1 ? 'IO + stable' : 'Active writes' }}</span>
              </div>
              <label for="stage-path-0" class="sr-only">Path for {{ stageLabel(0) }}</label>
              <input id="stage-path-0" v-model="config.stages[0]" type="text" :placeholder="`Path for ${stageLabel(0)}`" class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 font-mono text-sm focus:border-indigo-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-indigo-500/20 dark:border-gray-700 dark:bg-gray-950 dark:focus:bg-gray-900" />
            </div>
            <div class="flex items-center gap-2">
              <span class="text-xs text-gray-500 dark:text-gray-400">Stage 0</span>
              <button
                type="button"
                @click="removeStage(0)"
                class="rounded-md border border-red-300 bg-white px-2 py-1 text-xs font-medium text-red-700 shadow-sm transition-all hover:-translate-y-0.5 hover:border-red-400 hover:bg-red-50 hover:text-red-800 hover:shadow-md focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500 active:translate-y-0 dark:border-red-900 dark:bg-gray-900 dark:text-red-300 dark:hover:border-red-700 dark:hover:bg-red-950 dark:hover:text-red-200 dark:focus-visible:outline-red-400"
              >
                Remove
              </button>
            </div>
          </div>
          <div v-if="stageSpaceFor(config.stages[0])" class="mt-4 border-t border-gray-100 pt-3 dark:border-gray-800">
            <div class="mb-2 flex flex-wrap justify-between gap-2 text-xs text-gray-600 dark:text-gray-400">
              <span>{{ formatBytes(stageSpaceFor(config.stages[0])?.directoryBytes) }} used</span>
              <span>{{ formatBytes(stageSpaceFor(config.stages[0])?.effectiveCapacityBytes) }} usable</span>
            </div>
            <div class="h-2 overflow-hidden rounded-full bg-gray-100 dark:bg-gray-800" role="img" :aria-label="`${stageLabel(0)} storage usage: ${usagePercent(stageSpaceFor(config.stages[0]))}%`">
              <div class="h-full rounded-full bg-gradient-to-r from-emerald-500 to-teal-500 transition-[width] dark:from-emerald-400 dark:to-teal-400" :style="{ width: `${usagePercent(stageSpaceFor(config.stages[0]))}%` }"></div>
            </div>
          </div>
        </div>
        <PipelineEdgeCard
          v-if="config.stages.length > 1"
          class="my-4"
          mode="activity"
          :source-label="stageLabel(0)"
          :destination-label="stageLabel(1)"
          @insert="insertStageAt(1)"
          @reconfigure="openOutputSettings(0)"
          @manual="openManualMigration(0)"
        />

        <!-- Display intermediate stages (excluding first and last) -->
        <template v-for="(stage, index) in config.stages.slice(1, -1)" :key="index + 1">
          <article class="my-4 rounded-xl border border-indigo-200 bg-white p-4 shadow-sm transition-shadow hover:shadow-md dark:border-indigo-900 dark:bg-gray-900">
            <div class="flex flex-wrap items-start gap-3">
              <span class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-indigo-100 text-sm font-bold text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300" aria-hidden="true">{{ String(index + 1).padStart(2, '0') }}</span>
              <div class="min-w-0 flex-1">
                <div class="mb-2 flex flex-wrap items-center gap-2">
                  <h3 class="font-semibold">{{ stageLabel(index + 1) }}</h3>
                  <span class="rounded-full bg-indigo-50 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300">Intermediate tier</span>
                  <span v-if="unsavedStageIndexes.has(index + 1)" class="rounded-full bg-amber-100 px-2 py-0.5 text-[11px] font-semibold text-amber-800 dark:bg-amber-950 dark:text-amber-300">New</span>
                </div>
                <label :for="`stage-path-${index + 1}`" class="sr-only">Path for {{ stageLabel(index + 1) }}</label>
                <input :id="`stage-path-${index + 1}`" v-model="config.stages[index + 1]" type="text" :placeholder="`Path for ${stageLabel(index + 1)}`" class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 font-mono text-sm focus:border-indigo-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-indigo-500/20 dark:border-gray-700 dark:bg-gray-950 dark:focus:bg-gray-900" />
              </div>
              <div class="flex items-center gap-2">
                <button type="button" @click="removeStage(index + 1)" class="rounded-lg border border-red-200 px-3 py-2 text-xs font-semibold text-red-700 hover:border-red-300 hover:bg-red-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500 dark:border-red-900 dark:text-red-300 dark:hover:bg-red-950 dark:focus-visible:outline-red-400" :aria-label="`Remove ${stageLabel(index + 1)}`">
                  Remove
                </button>
              </div>
            </div>
            <div v-if="stageSpaceFor(stage)" class="mt-4 border-t border-gray-100 pt-3 dark:border-gray-800">
              <div class="mb-2 flex flex-wrap justify-between gap-2 text-xs text-gray-600 dark:text-gray-400">
                <span>{{ formatBytes(stageSpaceFor(stage)?.directoryBytes) }} used</span>
                <span>{{ formatBytes(stageSpaceFor(stage)?.effectiveCapacityBytes) }} usable</span>
              </div>
              <div class="h-2 overflow-hidden rounded-full bg-gray-100 dark:bg-gray-800" role="img" :aria-label="`${stageLabel(index + 1)} storage usage: ${usagePercent(stageSpaceFor(stage))}%`">
                <div class="h-full rounded-full bg-gradient-to-r from-indigo-500 to-violet-500 transition-[width] dark:from-indigo-400 dark:to-violet-400" :style="{ width: `${usagePercent(stageSpaceFor(stage))}%` }"></div>
              </div>
            </div>
          </article>
          <PipelineEdgeCard
            v-if="index + 2 < config.stages.length"
            class="my-4"
            mode="schedule"
            :source-label="stageLabel(index + 1)"
            :destination-label="stageLabel(index + 2)"
            @insert="insertStageAt(index + 2)"
            @reconfigure="openOutputSettings(index + 1)"
            @manual="openManualMigration(index + 1)"
          />
        </template>

        <!-- Display last stage (N) as a special visual element -->
        <div v-if="config.stages.length > 1" class="my-4 rounded-xl border border-amber-200 bg-white p-4 shadow-sm dark:border-amber-900 dark:bg-gray-900">
          <div class="flex flex-wrap items-start gap-3">
            <span class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-amber-100 text-sm font-bold text-amber-700 dark:bg-amber-950 dark:text-amber-300" aria-hidden="true">{{ String(config.stages.length - 1).padStart(2, '0') }}</span>
            <div class="min-w-0 flex-1">
              <div class="mb-2 flex flex-wrap items-center gap-2">
                <h3 class="font-semibold">{{ stageLabel(config.stages.length - 1) }}</h3>
                <span class="rounded-full bg-amber-50 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-amber-700 dark:bg-amber-950 dark:text-amber-300">Final tier</span>
                <span v-if="unsavedStageIndexes.has(config.stages.length - 1)" class="rounded-full bg-amber-100 px-2 py-0.5 text-[11px] font-semibold text-amber-800 dark:bg-amber-950 dark:text-amber-300">New</span>
              </div>
              <label :for="`stage-path-${config.stages.length - 1}`" class="sr-only">Path for {{ stageLabel(config.stages.length - 1) }}</label>
              <input :id="`stage-path-${config.stages.length - 1}`" v-model="config.stages[config.stages.length - 1]" type="text" :placeholder="`Path for ${stageLabel(config.stages.length - 1)}`" class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2 font-mono text-sm focus:border-indigo-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-indigo-500/20 dark:border-gray-700 dark:bg-gray-950 dark:focus:bg-gray-900" />
            </div>
            <div class="flex items-center gap-2">
              <span class="text-xs text-gray-500 dark:text-gray-400">Stage {{ config.stages.length - 1 }}</span>
              <button
                type="button"
                @click="removeStage(config.stages.length - 1)"
                class="rounded-md border border-red-300 bg-white px-2 py-1 text-xs font-medium text-red-700 shadow-sm transition-all hover:-translate-y-0.5 hover:border-red-400 hover:bg-red-50 hover:text-red-800 hover:shadow-md focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500 active:translate-y-0 dark:border-red-900 dark:bg-gray-900 dark:text-red-300 dark:hover:border-red-700 dark:hover:bg-red-950 dark:hover:text-red-200 dark:focus-visible:outline-red-400"
              >
                Remove
              </button>
            </div>
          </div>
          <div v-if="stageSpaceFor(config.stages[config.stages.length - 1])" class="mt-4 border-t border-gray-100 pt-3 dark:border-gray-800">
            <div class="mb-2 flex flex-wrap justify-between gap-2 text-xs text-gray-600 dark:text-gray-400">
              <span>{{ formatBytes(stageSpaceFor(config.stages[config.stages.length - 1])?.directoryBytes) }} used</span>
              <span>{{ formatBytes(stageSpaceFor(config.stages[config.stages.length - 1])?.effectiveCapacityBytes) }} usable</span>
            </div>
            <div class="h-2 overflow-hidden rounded-full bg-gray-100 dark:bg-gray-800" role="img" :aria-label="`${stageLabel(config.stages.length - 1)} storage usage: ${usagePercent(stageSpaceFor(config.stages[config.stages.length - 1]))}%`">
              <div class="h-full rounded-full bg-gradient-to-r from-amber-500 to-orange-500 transition-[width] dark:from-amber-400 dark:to-orange-400" :style="{ width: `${usagePercent(stageSpaceFor(config.stages[config.stages.length - 1]))}%` }"></div>
            </div>
          </div>
        </div>
        </section>

        <StageOutputModal
          v-if="outputSettingsStage !== null"
          :source-label="outputSourceLabel"
          :destination-label="outputDestinationLabel"
          :destination-path="outputDestinationPath"
          :mode="outputSettingsStage === 0 ? 'activity' : 'schedule'"
          @close="outputSettingsStage = null"
        />
        <ManualMigrationModal
          v-if="manualMigrationStage !== null"
          :start-index="manualMigrationStage"
          :stage-paths="config.stages"
          :stage-labels="stageLabels"
          :stage-spaces="stageSpaces?.stages ?? []"
          :can-migrate="canMigrate"
          @close="manualMigrationStage = null"
        />

        <details class="mt-6 rounded-xl border border-gray-200 dark:border-gray-700">
          <summary class="cursor-pointer list-none rounded-xl px-4 py-3 font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400">
            <span class="mr-2 text-gray-400" aria-hidden="true">⚙</span> Advanced settings
            <span class="ml-2 text-sm font-normal text-gray-500 dark:text-gray-400">Port and mount point</span>
          </summary>
          <div class="grid grid-cols-1 gap-4 border-t border-gray-200 p-4 sm:grid-cols-2 dark:border-gray-700">
            <div>
              <label for="config-port" class="mb-1 block text-sm font-medium">API port</label>
              <input id="config-port" v-model.number="config.port" type="number" min="1" max="65535" class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20 dark:border-gray-600 dark:bg-gray-900" />
            </div>
            <div>
              <label for="config-mount" class="mb-1 block text-sm font-medium">Mount point</label>
              <input id="config-mount" v-model="config.mount_point" type="text" class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20 dark:border-gray-600 dark:bg-gray-900" />
            </div>
          </div>
        </details>

        <div v-if="saveError" class="mt-4 rounded-lg border border-red-300 bg-red-50 px-4 py-3 text-sm text-red-800 dark:border-red-900 dark:bg-red-950 dark:text-red-200" role="alert">
          Failed to save configuration: {{ saveError }}
        </div>
        <div class="mt-6 flex flex-wrap items-center justify-end gap-3">
          <button type="button" @click="store.loadStages(true)" :disabled="stagesLoading" class="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 disabled:opacity-50 dark:border-gray-700 dark:hover:bg-gray-800 dark:focus-visible:outline-indigo-400">
            {{ stagesLoading ? 'Refreshing space…' : 'Refresh space' }}
          </button>
          <button type="button" @click="saveConfiguration" :disabled="saving" class="rounded-lg bg-indigo-600 px-4 py-2 text-sm font-semibold text-white hover:bg-indigo-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 disabled:opacity-50 dark:bg-indigo-500 dark:hover:bg-indigo-600 dark:focus-visible:outline-indigo-400">
            {{ saving ? 'Saving…' : 'Save configuration' }}
          </button>
        </div>
      </template>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useCloned } from '@vueuse/core';
import { storeToRefs } from 'pinia';
import DiskUsageCard from '@/components/DiskUsageCard.vue';
import IODirectoryCard from '@/components/IODirectoryCard.vue';
import ServiceStorageCard from '@/components/ServiceStorageCard.vue';
import PipelineEdgeCard from '@/components/PipelineEdgeCard.vue';
import StageOutputModal from '@/components/StageOutputModal.vue';
import ManualMigrationModal from '@/components/ManualMigrationModal.vue';
import { usePipelineStore } from '@/stores/pipeline';
import { useStages } from '@/composables/useStages';
import { useHealth } from '@/composables/useHealth';

const store = usePipelineStore();
const {
  config: savedConfig,
  configLoading: loading,
  configError: error,
  stagesLoading,
  stageSpaces,
  saving,
  stagesError: stageSpaceError,
  saveError,
} = storeToRefs(store);
const { cloned: config } = useCloned(savedConfig, { deep: true });
const { stageSpaceFor, usagePercent, formatBytes } = useStages();
const unsavedStageIndexes = ref(new Set<number>());
const outputSettingsStage = ref<number | null>(null);
const manualMigrationStage = ref<number | null>(null);
const { canQuery } = useHealth();
const canMigrate = computed(() => canQuery('migrateEndpoint'));
const stageLabels = computed(() => config.value?.stages.map((_, index) => stageLabel(index)) ?? []);

const outputSourceLabel = computed(() => {
  return stageLabel(outputSettingsStage.value ?? 0);
});
const outputDestinationIndex = computed(() => {
  const index = (outputSettingsStage.value ?? 0) + 1;
  return Math.min(index, Math.max((config.value?.stages.length ?? 1) - 1, 0));
});
const outputDestinationLabel = computed(() => stageLabel(outputDestinationIndex.value));
const outputDestinationPath = computed(() => config.value?.stages[outputDestinationIndex.value] ?? '');

onMounted(() => {
  void store.loadConfig();
  void store.loadStages();
});

function stageLabel(index: number) {
  if (!config.value) return `Stage ${index}`;
  const isFirst = index === 0;
  const isLast = index === config.value.stages.length - 1;
  if (isFirst && isLast) return `IO stage (Stage ${index})`;
  if (isFirst) return `IO stage (Stage ${index})`;
  if (isLast) return `Output stage (Stage ${index})`;
  return `Stage ${index}`;
}

watch(savedConfig, (newConfig) => {
  if (newConfig) {
    config.value = { ...newConfig, stages: [...newConfig.stages] };
    unsavedStageIndexes.value.clear();
  }
}, { immediate: true });

function insertStageAt(index: number) {
  if (!config.value) return;
  const stages = [...config.value.stages];
  stages.splice(index, 0, '');
  unsavedStageIndexes.value = new Set(
    [...unsavedStageIndexes.value].map((pendingIndex) =>
      pendingIndex >= index ? pendingIndex + 1 : pendingIndex,
    ),
  );
  unsavedStageIndexes.value.add(index);
  config.value = { ...config.value, stages };
}

function openOutputSettings(stageIndex: number) {
  outputSettingsStage.value = stageIndex;
}

function openManualMigration(stageIndex: number) {
  manualMigrationStage.value = stageIndex;
}

function removeStage(index: number) {
  if (!config.value || config.value.stages.length <= 1) return;
  const stages = [...config.value.stages];
  stages.splice(index, 1);
  outputSettingsStage.value = null;
  unsavedStageIndexes.value = new Set(
    [...unsavedStageIndexes.value]
      .filter((pendingIndex) => pendingIndex !== index)
      .map((pendingIndex) => pendingIndex > index ? pendingIndex - 1 : pendingIndex),
  );
  config.value = { ...config.value, stages };
}

async function saveConfiguration() {
  if (!config.value) return;
  const saved = await store.saveConfig(config.value);
  if (saved) {
    unsavedStageIndexes.value.clear();
    await store.loadStages(true);
  }
}
</script>
