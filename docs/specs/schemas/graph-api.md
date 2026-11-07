# DAG Graph API Payloads & JSON Schemas

This document defines the JSON payload structures and schemas for the graph-centric control API (`/api/v1/graph`, `/api/v1/graph/nodes`, `/api/v1/graph/edges`).

---

## 1. Complete Graph Payload (`GET /api/v1/graph` response & `PUT /api/v1/graph` request)

```json
{
  "version": "1.0",
  "nodes": [
    {
      "id": "stage_null",
      "type": "root",
      "name": "Stage Null (Active I/O)",
      "path": "/var/lib/pipeline-fs/null",
      "aliases": ["null", "active"],
      "metadata": { "role": "active_io" }
    },
    {
      "id": "stage_0",
      "type": "stage",
      "name": "Stage 0 (Staging Anchor)",
      "path": "/var/lib/pipeline-fs/stage0",
      "aliases": ["base"],
      "metadata": { "role": "base_staging" }
    },
    {
      "id": "input_var_1",
      "type": "input-variant",
      "name": "Ingest Variant A",
      "path": "/var/lib/pipeline-fs/input_var_a",
      "aliases": ["ingest-a"],
      "subPipeline": {
        "stages": ["input_null", "input_0"]
      }
    },
    {
      "id": "output_var_1",
      "type": "output-variant",
      "name": "Archival Export Variant",
      "path": "/var/lib/pipeline-fs/export_var",
      "aliases": ["export"],
      "metadata": { "branchSource": "stage_0" }
    }
  ],
  "edges": [
    {
      "id": "edge_null_0",
      "source": "stage_null",
      "target": "stage_0",
      "triggers": [
        {
          "type": "stable_duration",
          "parameters": { "threshold_seconds": 3600 }
        }
      ],
      "conditions": [
        {
          "expression": "size_bytes > 0"
        },
        {
          "expression": "!is_locked"
        }
      ],
      "actions": [
        {
          "type": "duplicate",
          "fallback": {
            "strategy": "fail",
            "notification": true
          }
        }
      ]
    }
  ]
}
```

---

## 2. Node Retrieval & Mutation Payloads (`GET / PUT /api/v1/graph/nodes/<id>`)

```json
{
  "type": "stage",
  "name": "Stage 1 (Warm Tier)",
  "path": "/var/lib/pipeline-fs/stage1",
  "aliases": ["warm-tier"],
  "metadata": {}
}
```

---

## 3. Edge Retrieval & Mutation Payloads (`GET / PUT /api/v1/graph/edges/<id>`)

```json
{
  "id": "edge_0_1",
  "source": "stage_0",
  "target": "stage_1",
  "triggers": [
    {
      "type": "ttl",
      "parameters": { "age_days": 7 }
    }
  ],
  "conditions": [
    {
      "expression": "true"
    }
  ],
  "actions": [
    {
      "type": "demote",
      "fallback": {
        "strategy": "push",
        "maxDepth": 3,
        "pushHandler": "default_cascade"
      }
    }
  ]
}
```
