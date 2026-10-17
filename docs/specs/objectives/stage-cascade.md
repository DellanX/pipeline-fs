# Stage Cascade Specification

This specification details the cascading lookup resolution and POSIX operation routing semantics across stages in `pipeline-fs`.

---

## 1. Stage Hierarchy & Order

`pipeline-fs` configures an ordered array of stage directories:
```
Stages = [Stage_null, Stage_0, Stage_1, ..., Stage_N]
```

Where:
- `Stage_null` (index -1 or 0 in configuration) represents volatile / active I/O.
- `Stage_0` represents the base staging layer.
- `Stage_1` through `Stage_N` represent progressively lower archival storage tiers.

---

## 2. Path Resolution & Cascading Lookup

### 2.1 Lookup Algorithm
When a path $P$ is requested by a VFS operation:

1. Iterate $i$ from $0$ to $N$ across `Stages`:
   a. Check if a whiteout entry exists for $P$ at `Stages[i]`. If a whiteout is present, stop search and return `ENOENT` (File Not Found).
   b. Check if path $P$ exists at `Stages[i]`.
   c. If $P$ exists at `Stages[i]`, return `Stages[i]/P` as the primary target.
2. If $P$ is not found in any stage, return `ENOENT`.

### 2.2 Directory Merging (`readdir`)
When listing a directory $D$:
1. Query directory $D$ across all stages from $0$ to $N$.
2. Accumulate child entry names into a set, suppressing any entry name that has an associated whiteout marker in an upper stage.
3. Return the consolidated directory entry listing to the client.

---

## 3. POSIX Operation Handling

### 3.1 Read Operations (`read`, `getattr`, `access`)
- Resolved using the top-down lookup algorithm.
- Data is read directly from the first stage in the hierarchy where the entry exists.

### 3.2 Write Operations (`write`, `truncate`, `utimens`)
- **Active Stage Redirection**: All new file creations and modifications are directed to `Stage_null`.
- **Copy-Up on Write**: If a file exists in a lower stage ($k > \text{null}$) but not in `Stage_null`, a copy-up to `Stage_null` is triggered before performing the write operation, preserving copy-on-write semantics.

### 3.3 Deletion & Renaming (`unlink`, `rmdir`, `rename`)
- **Unlink / Rmdir (Tombstone Marking)**:
  - Deleting a file does not instantly purge physical copies residing in higher archival tiers ($1 \dots N$). Instead, it creates a tombstone (whiteout marker, e.g. `.wh.<filename>`) in `Stage_null`.
  - Tombstone propagation: When demotion or background consolidation runs, whiteouts/tombstones propagate downward through the stages strictly in order ($0 \to N$), ensuring deleted files are eventually purged from each stage as demotion passes through.
- **Rename**:
  - Create whiteout marker for the source name in upper stages if it exists in lower stages.
  - Write target file or directory under new name into `Stage_null`.
