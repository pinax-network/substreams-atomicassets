use substreams_entity_change::{tables::Tables, pb::entity::EntityChanges};
use substreams::{errors::Error};

use crate::utils;
use crate::atomicassets::*;

#[substreams::handlers::map]
fn graph_out(anyevents: AnyEvents) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for anyevent in anyevents.items {
        match anyevent.event {
            Some(any_event::Event::AssetsTableItem(event)) => {
                let asset_id = &event.asset_id.to_string();
                tables
                    .create_row("Assets", asset_id)
                    .set_bigint("asset_id", asset_id)
                    .set("owner", &event.owner)
                    .set("collection_name", &event.collection_name)
                    .set_bigint("template_id", &event.template_id.to_string());
            },
            Some(any_event::Event::CollectionsTableItem(event)) => {
                let collection_name = &event.collection_name;
                tables
                    .create_row("Collections", collection_name)
                    .set("collection_name", collection_name)
                    .set("author", &event.author)
                    .set("allow_notify", &event.allow_notify)
                    .set("authorized_accounts", &event.authorized_accounts)
                    .set("notify_accounts", &event.notify_accounts)
                    .set("market_fee", &event.market_fee.to_string());
            },
            Some(any_event::Event::SchemasTableItem(event)) => {
                let schema_name = &event.schema_name;
                let (names, dtypes) = utils::get_formats(&event);
                tables
                    .create_row("Schemas", schema_name)
                    .set("schema_name", schema_name)
                    .set("format.name", names)
                    .set("format.dtype", dtypes)
                    .set("collection_name", &event.collection_name);
            },
            Some(any_event::Event::TemplatesTableItem(event)) => {
                let template_id = &event.template_id.to_string();
                tables
                    .create_row("Templates", template_id)
                    .set_bigint("template_id", template_id)
                    .set("schema_name", &event.schema_name)
                    .set("transferable", &event.transferable)
                    .set("burnable", &event.burnable)
                    .set("max_supply", event.max_supply)
                    .set("issued_supply", event.issued_supply)
                    .set("collection_name", &event.collection_name);
            },
            Some(any_event::Event::BalancesTableItem(event)) => {
                let owner = &event.owner.to_string();
                tables
                    .create_row("Balances", owner)
                    .set("owner", owner)
                    .set("quantities", &event.quantities);
            },
            Some(any_event::Event::OffersTableItem(event)) => {
                let offer_id = &event.offer_id.to_string();
                // convert Vec<u64> to Vec<String>
                let sender_asset_ids = utils::vec_to_string(event.sender_asset_ids);
                let recipient_asset_ids = utils::vec_to_string(event.recipient_asset_ids);
                tables
                    .create_row("Offers", offer_id)
                    .set_bigint("offer_id", offer_id)
                    .set("sender", &event.sender)
                    .set("recipient", &event.recipient)
                    .set("sender_asset_ids", sender_asset_ids)
                    .set("recipient_asset_ids", recipient_asset_ids)
                    .set("memo", &event.memo)
                    .set("ram_payer", &event.ram_payer);
            },
            Some(any_event::Event::TransferItem(event)) => {
                let collection_name = &event.collection_name;
                let asset_ids = utils::vec_to_string(event.asset_ids);
                tables
                    .create_row("Transfers", collection_name)
                    .set("collection_name", collection_name)
                    .set("from", &event.from)
                    .set("to", &event.to)
                    .set("asset_ids", asset_ids)
                    .set("memo", &event.memo);
            },
            _ => {continue}
        }   
    }
    Ok(tables.to_entity_changes())
}