# Pipeline-FS Architecture

This document maps the capabilities and goals specified in [`OBJECTIVE.md`](./OBJECTIVE.md) to technical components, software subsystem boundaries, and execution models.

---

## 1. System Overview & Layering

`pipeline-fs` is built as a user-space FUSE (Filesystem in Userspace) daemon implemented in Rust using `fuser` and `tokio`.

```
                  +-----------------------------------+
                  |        POSIX VFS / FUSE Kernel    |
                  +-----------------------------------+
                                    │
                                    ▼
                  +-----------------------------------+
                  |         FUSE Interface Layer      |
                  |          (src/fs/mod.rs)          |
                  +-----------------------------------+
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         ▼                          ▼                          ▼
+------------------+      +-------------------+      +------------------+
|  Stage Resolver  |      | Data Migration    |      | Control Interface|
|  (src/stage/)    |      | Engine            |      | (src/control/)   |
|                  |      | (src/migration/)  |      |                  |
+------------------+      +-------------------+      +------------------+
         │                          │                          │
         ▼                          ▼                          ▼
+-----------------------------------------------------------------------+
|                       Underlying Stage Storage                        |
|   [/stage_null]   [/stage_0]   [/stage_1]   ...   [/stage_N]          |
+-----------------------------------------------------------------------+
```

---

## 2. Core Subsystems

### 2.1 FUSE Interface Layer (`src/fs/`)
- Intercepts POSIX filesystem operations (`lookup`, `read`, `write`, `readdir`, `unlink`, `rename`, `setxattr`, `getxattr`).
- Translates inode-based FUSE requests to path-based or descriptor-based calls routed through the Stage Resolver.
- Handles inode-to-stage-path mappings and attribute caching.

### 2.2 Stage Cascade Resolver (`src/stage/`)
- Manages the ordered list of underlying stage root paths (`null`, `0`, `1`, ..., `N`).
- Implements cascading top-down lookup: queries stages sequentially from `null` down to `N` to locate files.
- Handles whiteout files (tombstones) to preserve deletion semantics across lower tiers.
- Routes write operations to the active write stage (`null` or `0` depending on policy).

### 2.3 Data Migration Engine (`src/migration/`)
- Executes asynchronous data movement tasks:
  - **Duplication (`null` → `0`)**: Copies stable files from `null` to `0`.
  - **Inter-stage Demotion ($i \to i+1$)**: Performs two-phase copy-and-remove lifecycle transitions.
  - **N-to-0 Stage Shifting**: Executes topology reconfigurations and stage index shifts from stage $N$ down to stage $0$ to safeguard against premature data merging or race conditions.
  - **Tombstone Propagation**: Propagates deletion tombstones downward through the stage cascade from $0 \to N$ during demotion/cleanup cycles.
- Manages concurrency control, read/write locking during active transfers, and crash-recovery journal state.

### 2.4 Control & Policy Interface (`src/control/`)
- **Web / API Control UI (Vue 3 + Tailwind CSS)**: 
  - A modern reactive dashboard built with **Vue 3** (Composition API / SFCs) and styled with **Tailwind CSS** for rapid styling, layout adjustments, and responsive design.
  - Communicates with the backend daemon via a hybrid standard protocol:
    - **REST API / JSON**: Used for fetching static configuration, stage topology metadata, initiating manual migrations, and updating policy rules.
    - **Server-Sent Events (SSE)** / **WebSockets**: Used for real-time streaming of stage utilization metrics, active migration progress, task logs, and live filesystem file-browser updates.
- **Extended Attribute (`xattr`) Interceptor**: Intercepts `user.pipeline.*` xattrs to trigger explicit file demotion or inspect stage metadata.
- **Control IPC Channel / CLI**: Provides a UNIX domain socket or CLI command channel for daemon management and manual batch triggers.
- **Background Policy Daemon**: Periodically evaluates age, TTL, and stage capacity usage to schedule automatic demotions.

---

## 3. Subsystem Detailed Specifications

Detailed objective specifications describing operational logic and algorithms are maintained in dedicated sub-documents:

- [`docs/specs/objectives/stage-cascade.md`](./docs/specs/objectives/stage-cascade.md) — Stage lookup resolution, POSIX operation semantics, and whiteout handling.
- [`docs/specs/objectives/data-migration.md`](./docs/specs/objectives/data-migration.md) — Migration state machine, two-phase copy-and-delete protocol, locking, and crash safety.
- [`docs/specs/objectives/control-interface.md`](./docs/specs/objectives/control-interface.md) — Control interfaces, xattr triggers, socket IPC protocols, and background automation policies.

---

## 4. Code & Directory Structure

The repository follows a mirrored structure between source, documentation, and tests:

```
pipeline-fs/
├── OBJECTIVE.md
├── ARCHITECTURE.md
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── fs/
│   ├── stage/
│   ├── migration/
│   └── control/
├── docs/
│   ├── specs/
│   │   └── objectives/
│   │       ├── stage-cascade.md
│   │       ├── data-migration.md
│   │       └── control-interface.md
│   ├── fs/
│   ├── stage/
│   ├── migration/
│   └── control/
└── tests/
    ├── specs/
    ├── fs/
    ├── stage/
    ├── migration/
    └── control/
```
