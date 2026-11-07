# Pipeline-FS Configuration Schema (YAML)

This document defines the YAML configuration schema for `pipeline-fs`, outlining the mandatory root (`Stage_null`) and anchor (`Stage_0`) stages, the default connection edge between them, optional user-defined nodes (stages, input variants, output variants), and custom edges with automatic ID resolution.

---

## Example `config.yaml`

```yaml
server:
  mount_point: "/tmp/test-mount"
  port: 8080

# Mandatory core stages always present in the pipeline
pipeline:
  root:
    path: "/var/lib/pipeline-fs/null"
    name: "Stage Null (Active I/O)"
    aliases: ["null", "active"]

  anchor:
    path: "/var/lib/pipeline-fs/stage0"
    name: "Stage 0 (Staging Anchor)"
    aliases: ["base"]

  # Ingress policy / duplication rule between root (stage_null) and anchor (stage_0)
  anchoring:
    triggers:
      - type: "stable_duration"
        threshold_seconds: 3600
    conditions:
      - expression: "size_bytes > 0"
      - expression: "!is_locked"
    actions:
      - type: "duplicate"
        fallback:
          strategy: "fail"
          notification: true

  # Additional user-defined nodes (stages, input-variants, output-variants)
  nodes:
    - id: "stage_1"
      type: "stage"
      name: "Stage 1 (Warm Tier)"
      path: "/var/lib/pipeline-fs/stage1"
      aliases: ["warm"]

    - id: "input_var_1"
      type: "input-variant"
      name: "Ingest Variant A"
      path: "/var/lib/pipeline-fs/input_var_a"
      aliases: ["ingest-a"]

    - id: "output_var_1"
      type: "output-variant"
      name: "Archival Export Variant"
      path: "/var/lib/pipeline-fs/export_var"
      aliases: ["export"]

  # Additional edges between nodes (id is optional; if omitted, auto-generated as "<source>-><target>")
  edges:
    - source: "stage_0"
      target: "stage_1"
      triggers:
        - type: "ttl"
          age_days: 7
      actions:
        - type: "demote"
          fallback:
            strategy: "push"
            max_depth: 3
            push_handler: "default_cascade"
```

---

## Edge ID Default Resolution Logic
If the `id` field is omitted on an edge definition:
1. The configuration parser automatically generates an ID in the format `<source>-><target>` (e.g., `stage_0->stage_1`).
2. If multiple edges share the same source and target, a disambiguation suffix is appended (e.g., `stage_0->stage_1#2`).
