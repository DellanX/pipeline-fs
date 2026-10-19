import { createSharedComposable, useIntervalFn } from '@vueuse/core';
import { computed, ref } from 'vue';
import type { Capabilities } from './types/Capabilities.js';
import type { HealthData } from './types/HealthData.js';
import type { ServiceStatus } from './types/ServiceStatus.js';
import { useFetch } from './useFetch.js';

function createHealthState() {
    const lastChecked = ref<Date | null>(null);
    const { data, error, execute, isFinished, statusCode } = useFetch(
        '/health',
        { immediate: false, refetch: false },
    ).json<HealthData>();

    const connected = computed(
        () => isFinished.value && !error.value && statusCode.value === 200 && data.value !== null,
    );
    const isHealthy = computed(() => connected.value && data.value?.status === 'healthy');
    const capabilities = computed<Capabilities | null>(() =>
        connected.value ? data.value?.capabilities ?? null : null,
    );
    const status = computed<ServiceStatus>(() => ({
        connected: connected.value,
        isHealthy: isHealthy.value,
        isUnhealthy: connected.value && !isHealthy.value,
        lastChecked: lastChecked.value,
        healthData: connected.value ? data.value ?? null : null,
        capabilities: capabilities.value,
    }));

    let inFlight: Promise<void> | null = null;
    const refreshStatus = () => {
        if (inFlight) return inFlight;

        inFlight = (async () => {
            try {
                await execute();
            } finally {
                lastChecked.value = new Date();
                inFlight = null;
            }
        })();
        return inFlight;
    };

    const canQuery = (capability: keyof Capabilities) =>
        isHealthy.value && capabilities.value?.[capability] === true;

    useIntervalFn(() => void refreshStatus(), 30_000);
    void refreshStatus();

    return { status, connected, isHealthy, capabilities, refreshStatus, canQuery };
}

export const useHealth = createSharedComposable(createHealthState);
export default useHealth;