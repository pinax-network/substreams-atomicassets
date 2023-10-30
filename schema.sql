CREATE TABLE Assets
(
    asset_id     UInt64,
    scope        String,
    collection_name String,
    template_id  Int32,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (asset_id)
ORDER BY (asset_id, scope, template_id, collection_name);
