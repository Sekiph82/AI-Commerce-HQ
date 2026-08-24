export type RuntimeState = 'STOPPED' | 'STARTING' | 'HEALTHY' | 'DEGRADED' | 'STOPPING' | 'FAILED' | 'DISABLED';
export type RuntimeHealth = 'HEALTHY' | 'DEGRADED' | 'FAILED' | 'DISABLED';

export type RuntimeComponent = {
  componentId: string;
  displayName: string;
  kind: 'NATIVE_CORE' | 'SIDECAR' | 'LEGACY_COMMERCE';
  state: RuntimeState;
  health: RuntimeHealth;
  startedAt: string | null;
  lastHeartbeat: string | null;
  restartCount: number;
  lastError: string | null;
  ownership: string;
};

export type RuntimeStatus = {
  architectureMode: 'RUST_NATIVE_NO_SIDECAR' | string;
  sidecarEnabled: boolean;
  legacyCommerceRuntime: RuntimeComponent;
  components: RuntimeComponent[];
  lastError: string | null;
};
