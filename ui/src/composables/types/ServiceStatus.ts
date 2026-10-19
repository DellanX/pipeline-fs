import type { Capabilities } from './Capabilities.js';
import type { HealthData } from './HealthData.js';

export interface ServiceStatus {
  connected: boolean;
  isHealthy: boolean;
  isUnhealthy: boolean;
  lastChecked: Date | null;
  healthData: HealthData | null;
  capabilities: Capabilities | null;
}
