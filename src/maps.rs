use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicassets::*;

#[substreams::handlers::map]
fn map_assets(block: Block) -> Result<AssetsTableOperations, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "assets" { continue; }

            let data;
            if db_op.operation == 3 {
                data = match abi::AssetsS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::AssetsS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }
            
            // template_id == -1 means the table assets is being created for the scope
            // not necessary here because it does not contain complete information
            if data.template_id == -1 { continue; }

            items.push(AssetsTableOperation {
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
            });
        }
    }
    Ok(AssetsTableOperations { items })
}