use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicassets::*;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<AnyEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // db_ops
        for db_op in &trx.db_ops {
            let json_data;
            match db_op.operation{
                0 => { log::debug!("db operation Unknown: {}", db_op.operation);
                        continue;}
                1 | 2 => { json_data = db_op.new_data_json.as_str(); }
                3 => { json_data = db_op.old_data_json.as_str(); }
                _ => { log::debug!("db operation couldn't be matched : {}", db_op.operation);
                        continue;}
            }
    
            match db_op.table_name.as_str() {
                "assets" => {
                    let data;
                    data = match abi::AssetsS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::AssetsS : {}", error);
                            continue;
                        }
                    };
                    if data.template_id == -1 { continue; }
                    response.push(AnyEvent {
                        event: Some(any_event::Event::AssetsTableItem(
                            AssetsTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
                                scope: db_op.scope.clone(),
    
                                // data payload
                                asset_id: data.asset_id.clone(),
                                collection_name: data.collection_name.clone(),
                                schema_name: data.schema_name.clone(),
                                template_id: data.template_id.clone(),
                            }
                        ))
                    });
                },
                "collections" => {
                    let data;
                    data = match abi::CollectionsS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::CollectionsS : {}", error);
                            continue;
                        }
                    };
    
                    response.push(AnyEvent {
                        event: Some(any_event::Event::CollectionsTableItem(
                            CollectionsTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
    
                                // data payload
                                collection_name: data.collection_name.clone(),
                                author: data.author.clone(),
                                allow_notify: data.allow_notify.clone(),
                                authorized_accounts: data.authorized_accounts.clone(),
                                notify_accounts: data.notify_accounts.clone(),
                                market_fee: data.market_fee.clone(),
                            }
                        ))
                    });
                },
                "templates" => {
                    let data;
                    data = match abi::TemplatesS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::TemplatesS : {}", error);
                            continue;
                        }
                    };
    
                    response.push(AnyEvent {
                        event: Some(any_event::Event::TemplatesTableItem(
                            TemplatesTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
    
                                // data payload
                                template_id: data.template_id.clone(),
                                schema_name: data.schema_name.clone(),
                                transferable: data.transferable.clone(),
                                burnable: data.burnable.clone(),
                                max_supply: data.max_supply.clone(),
                                issued_supply: data.issued_supply.clone(),
                            }
                        ))
                    });
                },
                "schemas" => {
                    let data;
                    data = match abi::SchemasS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::SchemasS : {}", error);
                            continue;
                        }
                    };
    
                    let mut format = vec![];
                    for f in &data.format {
                        format.push(Format {
                            name: f.name.clone(),
                            dtype: f.r#type.clone(),
                        });
                    }
    
                    response.push(AnyEvent {
                        event: Some(any_event::Event::SchemasTableItem(
                            SchemasTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
    
                                // data payload
                                collection_name: db_op.scope.clone(),
                                schema_name: data.schema_name.clone(),
                                format: format,
                            }
                        ))
                    });
                },
                "offers" => {
                    let data;
                    data = match abi::OffersS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::OfferS : {}", error);
                            continue;
                        }
                    };
    
                    response.push(AnyEvent {
                        event: Some(any_event::Event::OffersTableItem(
                            OffersTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
    
                                // data payload
                                offer_id: data.offer_id.clone(),
                                sender: data.sender.clone(),
                                recipient: data.recipient.clone(),
                                sender_asset_ids: data.sender_asset_ids.clone(),
                                recipient_asset_ids: data.recipient_asset_ids.clone(),
                                memo: data.memo.clone(),
                                ram_payer: data.ram_payer.clone(),
                            }
                        ))
                    });
                },
                "balances" => {
                    let data;
                    data = match abi::BalancesS::try_from(json_data){
                        Ok(data) => data,
                        Err(error) => {
                            log::debug!("json data not decoded into atomicassets::BalancesS : {}", error);
                            continue;
                        }
                    };
    
                    response.push(AnyEvent {
                        event: Some(any_event::Event::BalancesTableItem(
                            BalancesTableOperation {
                                // trace information
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
    
                                // db operation 
                                db_operation: db_op.operation.clone(),
    
                                // data payload
                                owner: data.owner.clone(),
                                quantities: data.quantities.clone(),
                            }
                        ))
                    });
                },
                _ => {continue}
            }  
        }
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicassets" {continue}
            if action_trace.name != "logtransfer" { continue}
            match abi::Logtransfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    // filtering only atomicmarket related transfers
                    // if data.from != "atomicmarket" && data.to != "atomicmarket" {continue}
                    response.push(AnyEvent {
                        event: Some(any_event::Event::TransferItem( 
                            TransferEvent{
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                collection_name: data.collection_name,
                                from: data.from,
                                to: data.to,
                                asset_ids: data.asset_ids,
                                memo: data.memo,
                            }
                        ))
                    });
                }
                Err(error) => {
                    log::debug!("Failed to decode atomicassets::logtransfer: {}", error);
                    continue;
                }
           }
        }
    }
    Ok(AnyEvents { items: response })
}