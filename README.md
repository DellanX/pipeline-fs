# Pipeline-FS

A multi-stage cascading FUSE filesystem with tier demotion capabilities.

## Features

- Multi-stage cascading filesystem with configurable stages
- Tier demotion for background migration of files between stages
- RESTful API for configuration and management
- Real-time telemetry via Server-Sent Events (SSE)
- Health check endpoint for monitoring service status

## API Endpoints

### Core Endpoints

1. **`/api/v1/stages`**: Get/set stage hierarchy configuration (enforcing $N$-to-$0$ shifting).
2. **`/api/v1/policies`**: Get/update background migration policies.
3. **`/api/v1/files/browse?stage=<id>&path=<p>`**: Browse file entries per stage or flattened.
4. **`/api/v1/migrate`**: Trigger explicit migration actions.

### Monitoring Endpoints

5. **`/health`**: Health check endpoint that returns service status, version, timestamp, and a `capabilities` object. Capability flags indicate which API endpoints are currently implemented, allowing clients to gate requests from this single response.

### Real-time Events

6. **`/api/v1/events`** (SSE or WebSocket): Real-time event stream pushing storage usage telemetry, active background migration job statuses, in-flight demotion progress, and filesystem mutation events to update the Vue 3 reactive store instantly.

## UI Views & Capabilities

1. **Pipeline Status & Telemetry Dashboard**: Live storage utilization bars, file counts, and health indicators for stages `null` through $N$.
2. **Dynamic Stage & Policy Reconfigurator**: Interactive forms for modifying stage topologies and rules.
3. **Per-Stage & Flattened File Browser**: Switchable views to inspect files locked in specific stage tiers or view the unified flattened namespace with tombstone/whiteout badges.

## Getting Started

1. Create the required directories:
   ```bash
   mkdir -p /tmp/stage0 /tmp/stage1
   ```

2. Run the service:
   ```bash
   ./target/debug/pipeline-fs --mount-point /tmp/test-mount --stages /tmp/stage0 /tmp/stage1 --port 8080
   ```

3. Access the health check endpoint:
   ```bash
   curl http://localhost:8080/health
   ```

## Health Check Endpoint

The service exposes a `/health` endpoint that returns a JSON object with:
- `status`: Service status ("healthy")
- `version`: Current version of the service
- `timestamp`: Unix timestamp when the check was performed

Example response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": 1792296829
}
```