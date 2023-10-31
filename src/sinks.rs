use substreams_entity_change::{tables::Tables, pb::entity::EntityChanges};
use substreams::{errors::Error};

use crate::atomicassets::*;

#[substreams::handlers::map]
fn graph_out(events: AssetsTableOperations) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in events.items {
        let asset_id = &event.asset_id.to_string();

        tables
            .create_row("Assets", asset_id)
            .set_bigint("asset_id", asset_id)
            .set("scope", &event.scope)
            .set("collection_name", &event.collection_name)
            .set_bigint("template_id", &event.template_id.to_string());
    }
    Ok(tables.to_entity_changes())
}