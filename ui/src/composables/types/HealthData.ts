import type { Capabilities } from './Capabilities.js';

export type HealthData = {
  status: string;
  version: string;
  timestamp: number;
  capabilities: Capabilities;
};
