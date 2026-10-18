# Pipeline-FS Objectives

`pipeline-fs` is a multi-stage cascading filesystem designed to provide tiered data management with seamless data lifecycle migration. It presents a unified filesystem view backed by an ordered sequence of storage stages.

---

## 1. Core Vision & Goals

The primary goal of `pipeline-fs` is to decouple active file I/O operations from long-term persistence and tiered storage management.

Key objectives:
- **Cascading Stage Topology**: Present a single merged filesystem mount backed by an active I/O stage (`null`), a base staging stage (`0`), and $N$ demotion/archival stages ($1 \dots N$).
- **Active I/O Isolation**: Perform write and volatile I/O operations primarily in upper stages to maintain low latency and operational independence.
- **Stage Duplication (`null` → `0`)**: Copy stable data from the volatile/active stage (`null`) to the base staging stage (`0`), making it available as the source for downstream lifecycle transitions.
- **Ordered Inter-Stage Demotion ($i$ → $i+1$)**: Move data progressively from upper stages to lower stages ($0 \to 1 \to 2 \dots \to N$). Once data reaches stage $N$, it is considered fully stabilized and archived.
- **Atomic Two-Phase Demotion**: Guarantee that moving data from stage $i$ to stage $i+1$ consists of a copy step followed by a removal step from stage $i$, preventing data loss or duplication across stage boundaries.
- **Top-Down Cascading Read & Stat**: Resolve file read and metadata requests by querying stages sequentially from top (`null`) to bottom ($N$), returning the first matching entry.
- **Hybrid Demotion Triggers**: Support both explicit user/application-driven migration requests and automated background policy enforcement (e.g. age, capacity thresholds, time-to-live).
- **Tombstone File & Deleted File Tracking**: Deleting a file marks it as deleted in the `null` stage via tombstone/whiteout markers, propagating downward strictly through stage demotion/propagation ($0 \to N$) rather than immediately purging physical copies in higher tiers.
- **N-to-0 Stage Shifting Order**: Stage reconfigurations, topology adjustments, and migrations execute in a strict $N$-to-$0$ reverse order to prevent premature merging, race conditions, or data loss.
- **Control & Monitoring UI**: Provide a web/API control interface for viewing stage storage metrics, pipeline configuration, per-stage file browsing, flattened views, and dynamic stage/policy reconfiguration.

---

## 2. Stage Lifecycle & Topology

The filesystem operates on an ordered pipeline of stages:

```
[ Active Mount Point ]
        │
        ▼
   Stage `null`     (Active / Volatile I/O)
        │
        ▼ (Duplication)
   Stage `0`        (Base Staging Anchor)
        │
        ▼ (Demotion: Copy + Remove)
   Stage `1`        (Warm Tier)
        │
        ▼ (Demotion: Copy + Remove)
       ...
        │
        ▼ (Demotion: Copy + Remove)
   Stage `N`        (Cold / Archive Tier - Terminal Stage)
```

### 2.1 Stage Roles

1. **Stage `null` (Active Tier)**
   - The primary target for incoming file creation, modifications, and temporary writes.
   - Designed for high-performance I/O.
2. **Stage `0` (Base Staging Anchor)**
   - Receives duplicate copies of stable files from stage `null`.
   - Serves as the origin point for downstream stage-by-stage demotion.
3. **Stage $i$ ($1 \le i < N$) (Intermediate Tiers)**
   - Intermediate storage stages representing progressively colder or lower-cost storage media.
   - Data resides here temporarily until demoted to stage $i+1$.
4. **Stage $N$ (Terminal Archive Tier)**
   - The final, static destination tier in the pipeline.
   - Data resting in stage $N$ is fully preserved and stationary.

---

## 3. Data Lifecycle & Movement Rules

### 3.1 Stage `null` to Stage `0` Duplication
- Data written to `null` is replicated/copied to stage `0` when marked or recognized as stable.
- The original copy in `null` may remain for active read caching or be cleaned up according to active-tier policy.

### 3.2 Inter-Stage Demotion ($i \to i+1$)
- Moving data from stage $i$ to $i+1$ MUST follow a two-step sequence:
  1. **Copy Phase**: Replicate file data and attributes from stage $i$ to stage $i+1$.
  2. **Removal Phase**: Unlink/delete the source data from stage $i$ once replication to stage $i+1$ is verified.
- Demotion must maintain data integrity and crash-safety throughout the transition.

---

## 4. User Interaction & Capabilities

Users and applications interacting with `pipeline-fs` should be able to:

1. **Mount & Navigate**: Mount the pipeline filesystem at any desired target directory and interact with it using standard filesystem operations (`ls`, `cat`, `cp`, `rm`, `mkdir`, etc.).
2. **Query Stage Attributes**: Inspect which stage currently holds the primary copy of any file or directory.
3. **Trigger Explicit Demotion**: Relegate specific files or directories to lower stages on demand via control interfaces or extended attributes.
4. **Configure Automated Policies**: Define background policy rules based on access timestamps, file age, or storage space utilization to automatically trigger demotion pipeline tasks.
