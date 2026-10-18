import { ref, computed } from 'vue';

export interface HealthData {
  status: string;
  version: string;
  timestamp: number;
}

export interface Capabilities {
  healthEndpoint: boolean;
  configEndpoint: boolean;
  stagesEndpoint: boolean;
  filesEndpoint: boolean;
  migrateEndpoint: boolean;
  eventsEndpoint: boolean;
  policiesEndpoint: boolean;
}

export interface ServiceStatus {
  isHealthy: boolean;
  isUnhealthy: boolean;
  lastChecked: Date | null;
  healthData: HealthData | null;
  capabilities: Capabilities | null;
}

export function useHealthCheck(apiUrl: string) {
  const healthData = ref<HealthData | null>(null);
  const capabilities = ref<Capabilities | null>(null);
  const lastChecked = ref<Date | null>(null);
  const isHealthy = ref(false);
  const isUnhealthy = ref(false);

  const status = computed<ServiceStatus>(() => ({
    isHealthy: isHealthy.value,
    isUnhealthy: isUnhealthy.value,
    lastChecked: lastChecked.value,
    healthData: healthData.value,
    capabilities: capabilities.value
  }));

  const checkHealth = async (): Promise<boolean> => {
    try {
      const response = await fetch(`${apiUrl}/health`);
      
      if (response.ok) {
        healthData.value = await response.json();
        isHealthy.value = true;
        isUnhealthy.value = false;
        lastChecked.value = new Date();
        return true;
      } else {
        isHealthy.value = false;
        isUnhealthy.value = true;
        lastChecked.value = new Date();
        return false;
      }
    } catch (error) {
      console.error('Health check failed:', error);
      isHealthy.value = false;
      isUnhealthy.value = true;
      lastChecked.value = new Date();
      return false;
    }
  };

  const checkCapabilities = async (): Promise<Capabilities | null> => {
    try {
      // Test which endpoints are available by making requests to them
      const capabilitiesResult: Capabilities = {
        healthEndpoint: true, // We already tested this
        configEndpoint: false,
        stagesEndpoint: false,
        filesEndpoint: false,
        migrateEndpoint: false,
        eventsEndpoint: false,
        policiesEndpoint: false
      };

      // Test config endpoint
      const configResponse = await fetch(`${apiUrl}/config`);
      capabilitiesResult.configEndpoint = configResponse.ok;

      // Test stages endpoint
      const stagesResponse = await fetch(`${apiUrl}/stages`);
      capabilitiesResult.stagesEndpoint = stagesResponse.ok;

      // Test files endpoint
      const filesResponse = await fetch(`${apiUrl}/files/browse`);
      capabilitiesResult.filesEndpoint = filesResponse.ok;

      // Test migrate endpoint
      const migrateResponse = await fetch(`${apiUrl}/migrate`);
      capabilitiesResult.migrateEndpoint = migrateResponse.ok;

      // Test events endpoint (this might be a streaming endpoint, so we'll test it differently)
      try {
        const eventsResponse = await fetch(`${apiUrl}/events`);
        capabilitiesResult.eventsEndpoint = true;
      } catch {
        capabilitiesResult.eventsEndpoint = false;
      }

      // Test policies endpoint
      const policiesResponse = await fetch(`${apiUrl}/policies`);
      capabilitiesResult.policiesEndpoint = policiesResponse.ok;

      capabilities.value = capabilitiesResult;
      return capabilitiesResult;
    } catch (error) {
      console.error('Capability check failed:', error);
      return null;
    }
  };

  const refreshStatus = async (): Promise<boolean> => {
    const healthSuccess = await checkHealth();
    if (healthSuccess) {
      await checkCapabilities();
    }
    return healthSuccess;
  };

  // Initialize status check
  refreshStatus();

  // Set up periodic checking (every 30 seconds)
  setInterval(() => {
    refreshStatus();
  }, 30000);

  // Function to gate queries based on service status
  const canQuery = (endpoint: keyof Capabilities): boolean => {
    if (!isHealthy.value) return false;
    if (!capabilities.value) return false;
    return capabilities.value[endpoint] || false;
  };

  return {
    status,
    refreshStatus,
    checkHealth,
    checkCapabilities,
    canQuery
  };
}