# Data Migration Specification

This specification details the lifecycle migration engine, two-phase copy-and-remove demotion protocol, and crash safety guarantees in `pipeline-fs`.

---

## 1. Migration Types & Edge Fallback Actions

`pipeline-fs` defines primary migration pathways and advanced edge conflict resolution rules:

1. **Duplication (`Stage_null` → `Stage_0`)**:
   - Replicates stable files from the active I/O stage (`null`) to the base staging stage (`0`).
   - Retains the source file in `Stage_null` (or transitions it according to active-tier cache policy).
2. **Sequential Inter-Stage Demotion (`Stage_i` → `Stage_i+1`)**:
   - Progresses data downward through archival stages ($0 \to 1 \to 2 \dots \to N$).
   - Completely removes source data from `Stage_i` once successfully replicated and verified in `Stage_i+1`.

### 1.1 Edge Conflict & Fallback Actions
When a trigger condition is met and the next stage (`Stage_i+1`) already contains data/files at the target path, one of three fallback actions executes:
1. **Fail**: Abort migration, throw an error, leave source untouched, and trigger user notification/event handlers.
2. **Merge**: Combine data from both stages (e.g., using timestamp-based latest-wins or recursive directory merging).
3. **Push**: Recursively push the next stage's contents downward. Configurable with a **maximum push depth** or delegated to node-specific **push event handlers**.

---

## 1.2 Transforms & Branching Variant Outputs

- **Custom Transform Actions**: Injectable transformation pipelines (splitting data across multiple nodes and recombining them for later nodes).
- **Branching Variant Outputs**: Transform outputs that branch from variant paths or intermediate stages to store manipulated datasets. These are strictly isolated and non-reconcilable with the main chain to preserve aggregate data structure integrity.

## 2. Two-Phase Copy-and-Remove Protocol ($i \to i+1$)

To guarantee zero data loss and prevent partial writes from corrupting archival stages, inter-stage demotion follows a strict two-phase state machine:

### Phase 1: Secure Copy
1. Lock source file at `Stage_i` against concurrent write modifications (exclusive read lock).
2. Create a temporary staging file at `Stage_i+1` (e.g. `.pipeline_migrating_<random>`).
3. Stream data blocks and replicate file extended attributes, permissions, and timestamps from `Stage_i` to the temporary file in `Stage_i+1`.
4. Perform an integrity check (e.g. size comparison or checksum verification).
5. Atomically rename the temporary file in `Stage_i+1` to its final path.
6. Issue an `fsync` on `Stage_i+1` to ensure non-volatile persistence.

### Phase 2: Source Removal & Cleanup
1. Verify that the target file exists and is intact in `Stage_i+1`.
2. Unlink/delete the source file from `Stage_i`.
3. Release migration locks.

---

## 3. Crash Safety & Recovery

- **In-Flight Migration Journal / State Log**: The migration engine maintains an active transaction log recording file paths currently undergoing migration.
- **Recovery on Startup**: Upon daemon restart, the migration worker scans the state log:
  - If a file is stuck in Phase 1 (Copy incomplete), the partial temp file in `Stage_i+1` is purged, leaving `Stage_i` untouched.
  - If a file completed Phase 1 but failed before Phase 2 removal, the verification check confirms the replica in `Stage_i+1` before safely unlinking the duplicate from `Stage_i`.

---

## 4. N-to-0 Stage Shifting Protocol

When system administrators reconfigure pipeline stages (e.g., adding, removing, or re-indexing stages between $0$ and $N$), operations MUST be executed in strict reverse order from stage $N$ down to stage $0$:

1. **Top-Down Lockout**: Acquire exclusive coordination locks starting from stage $N$ down to stage $0$.
2. **Reverse Shifting ($N \to 0$)**: Migrate or re-index stage directories starting at stage $N$, moving sequentially downward to stage $0$. This ensures that data residing in lower archival tiers is never prematurely overwritten, merged, or exposed to intermediate state inconsistencies before upper tiers are safely secured or reorganized.
3. **Atomic Topology Swap**: Commit the new stage index mapping and release locks from $0$ to $N$.
