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

## 4. Background Automated Policy Engine & Injectable Extensions

The background policy daemon periodically evaluates configurable rules:
- **Age / TTL Rules**: Automatically demote files from stage $i$ to $i+1$ if unmodified for a specified duration (e.g. > 7 days).
- **Capacity Thresholds**: Trigger demotion when active stages exceed high-water marks.
- **Injectable Triggers / Conditions / Actions**: Supports third-party/user-developed plugins, WASM modules, or script hooks for custom migration triggers, evaluation conditions, and action pipelines.

---

## 5. Control & Monitoring UI & File Browser (Vue 3 + Tailwind CSS)

`pipeline-fs` includes a reactive web control dashboard built with **Vue 3** (Composition API) and styled using **Tailwind CSS**.

#### 5.1 Communication Protocol
To cleanly handle both stateless command/configuration exchanges and real-time streaming (file monitoring, stage transfer progress, live metrics), the frontend and backend communicate via a standard protocol stack:
1. **REST JSON API (DAG Graph-Centric)**:
   - `/api/v1/config`: Read and update global pipeline configuration.
   - `/api/v1/graph`: 
     - `GET /api/v1/graph`: Returns the complete DAG topology (`nodes`, `edges`, `variants`, metadata, swimlane assignments, and fallback actions).
     - `PUT /api/v1/graph`: Atomically update or reconfigure the DAG nodes, edges, fallback behaviors, and variant paths.
   - `/api/v1/graph/nodes`: Add, update, or remove individual nodes (mainline stages, input sub-pipelines, or output variants).
   - `/api/v1/graph/edges`: Reconfigure edge triggers, conditions, actions, and fallback policies (Fail, Merge, Push with depth/handlers).
   - `GET /api/v1/stages`: Read filesystem capacity and filesystem identity for each configured stage and the IO directory. Each directory entry also reports recursive allocated file bytes and regular-file counts, plus effective directory capacity (filesystem total minus filesystem usage plus the directory size). The response includes total capacity across unique filesystems and a service-usage summary for stage directories only (excluding the IO view); overlapping stage paths or hard links may count the same data more than once.
   - `/api/v1/policies`: Get/update background migration policies.
   - `/api/v1/files/browse?stage=<id>&path=<p>`: Browse file entries per stage or flattened.
   - `/api/v1/migrate`: Trigger explicit migration actions.
2. **WebSockets / Server-Sent Events (SSE)**:
   - `/api/v1/events` (SSE or WebSocket): Real-time event stream pushing storage usage telemetry, active background migration job statuses, in-flight demotion progress, and filesystem mutation events to update the Vue 3 reactive store instantly.

#### 5.2 UI Views & Capabilities
1. **Interactive DAG Pipeline Topology & Graph Dashboard**:
   - **Mainline Center Layout**: The primary `Stage_null` acts as the central origin node, with the mainline (`Stage_0` through `Stage_N`) flowing horizontally to the right along the vertically centered axis.
   - **Input Variants (Left Swimlanes)**: Input variant paths added to the left of `Stage_null` operate as sub-pipelines (each with their own internal `null`, `0`, through $N$ stages). In the master graph view, each input variant appears as a consolidated sub-pipeline node that can be clicked to drill down into its internal pipeline-fs graph.
   - **Output Variants & Branching (Right/Vertical Swimlanes)**: Output variant paths can fork from any node in the mainline or stages. All output variants are positioned on separate vertical swimlanes.
   - **Interactive Graph Editing**: Intuitive UI controls enabling users to reconfigure edges and nodes, insert or remove nodes, and add/branch input or output variants on the fly.
