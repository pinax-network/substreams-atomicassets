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
              "int32": 9233
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
Version: v0.0.1
Modules:
----
Name: map_assets
Initial block: 0
Kind: map
Output Type: proto:antelope.atomicassets.v1.AssetsTableOperations
Hash: a10068218183fba448c923f3b72548fd1ab1ec91

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 707e9ad41e6a95e254bc924f64a6482a977db197
```