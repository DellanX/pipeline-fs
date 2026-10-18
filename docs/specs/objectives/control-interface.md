# Control Interface Specification

This specification details the hybrid control mechanisms—including extended attributes (`xattr`), IPC/CLI control channels, and background automated policy daemons—used to manage `pipeline-fs`.

---

## 1. Hybrid Control Architecture

`pipeline-fs` provides three complementary channels for inspecting stage states and triggering data migrations:
1. **Extended Attributes (`xattr`)**: File-level operations via standard system calls (`getxattr`, `setxattr`).
2. **CLI / IPC Daemon Control Socket**: Administrative control interface for batch migrations and status monitoring.
3. **Background Automation Policy Engine**: Rule-based worker thread evaluating TTL, file age, and capacity thresholds.

---

## 2. Extended Attribute (`xattr`) Control Interface

Files exposed through the mount point support custom `user.pipeline.*` extended attributes:

- `user.pipeline.stage`: (Read-Only) Returns the current stage index (`null`, `0`, `1`, ..., `N`) where the primary file data resides.
- `user.pipeline.migrate`: (Write-Only) Setting this attribute with a target stage number or keyword (e.g. `+1` or `next`) immediately schedules an explicit migration task for the file.
- `user.pipeline.replicate`: (Write-Only) Triggers immediate duplication from `null` to `0`.

---

## 3. CLI & IPC Control Daemon

A local UNIX domain socket or control pipe enables administrative CLI commands:
- `pipeline-fs status`: Report storage utilization and file counts per stage.
- `pipeline-fs migrate <path> --to <stage>`: Force explicit migration of a file or directory tree.
- `pipeline-fs flush`: Trigger immediate background policy evaluation.

---

## 4. Background Automated Policy Engine

The background policy daemon periodically evaluates configurable rules:
- **Age / TTL Rules**: Automatically demote files from stage $i$ to $i+1$ if unmodified for a specified duration (e.g. > 7 days).
- **Capacity Thresholds**: Trigger demotion when active stages (`null` or `0`) exceed a high-water mark percentage of disk utilization until a low-water mark is reached.

---

## 5. Control & Monitoring UI & File Browser (Vue 3 + Tailwind CSS)

`pipeline-fs` includes a reactive web control dashboard built with **Vue 3** (Composition API) and styled using **Tailwind CSS**.

#### 5.1 Communication Protocol
To cleanly handle both stateless command/configuration exchanges and real-time streaming (file monitoring, stage transfer progress, live metrics), the frontend and backend communicate via a standard protocol stack:
1. **REST JSON API**:
   - `/api/v1/stages`: Get/set stage hierarchy configuration (enforcing $N$-to-$0$ shifting).
   - `/api/v1/policies`: Get/update background migration policies.
   - `/api/v1/files/browse?stage=<id>&path=<p>`: Browse file entries per stage or flattened.
   - `/api/v1/migrate`: Trigger explicit migration actions.
2. **WebSockets / Server-Sent Events (SSE)**:
   - `/api/v1/events` (SSE or WebSocket): Real-time event stream pushing storage usage telemetry, active background migration job statuses, in-flight demotion progress, and filesystem mutation events to update the Vue 3 reactive store instantly.

#### 5.2 UI Views & Capabilities
1. **Pipeline Status & Telemetry Dashboard**: Live storage utilization bars, file counts, and health indicators for stages `null` through $N$.
2. **Dynamic Stage & Policy Reconfigurator**: Interactive forms for modifying stage topologies and rules.
3. **Per-Stage & Flattened File Browser**: Switchable views to inspect files locked in specific stage tiers or view the unified flattened namespace with tombstone/whiteout badges.
