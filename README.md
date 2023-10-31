# AtomicAssets Assets Table powered by **Substreams**

[![Build Status](https://github.com/pinax-network/substreams-atomicassets/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/substreams-atomicassets/actions/workflows/test.yml)
![Version](https://img.shields.io/github/v/release/pinax-network/substreams-atomicassets)
![License](https://img.shields.io/github/license/pinax-network/substreams-atomicassets)

> Asset ID, Template ID, Scope & Collection name

## Quick Start

```
gh repo clone pinax-network/substreams-atomicassets
cd substreams-atomicassets
make
make run        # runs the map_assets module for a block
```

### Mermaid graph

```mermaid
graph TD;
  map_assets[map: map_assets];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_assets;
  graph_out[map: graph_out];
  map_assets --> graph_out;

```
## Map Outputs

### `graph_out`

```json
{
  "entityChanges": [
      {
        "entity": "Assets",
        "id": "2199025347211",
        "operation": "OPERATION_CREATE",
        "fields": [
          {
            "name": "asset_id",
            "newValue": {
              "bigint": "2199025347211"
            }
          },
          {
            "name": "template_id",
            "newValue": {
              "int32": "9233"
            }
          },
          {
            "name": "scope",
            "newValue": {
              "string": "womplayitems"
            }
          },
          {
            "name": "collection_name",
            "newValue": {
              "string": "uplandislive"
            }
          }
        ]
      }
  ]
}
  
```

### Modules
```yaml
Package name: atomicassets
Version: v0.0.1-dev
Modules:
----
Name: map_assets
Initial block: 0
Kind: map
Output Type: proto:antelope.atomicassets.v1.AssetsTableOperations
Hash: 20352b9d3cfa3ac63fafccddc79dda5d4aaeab57

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: bf819973b362588d966e12b70bd6b9291253b02d
```