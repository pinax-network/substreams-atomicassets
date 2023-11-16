CREATE TABLE Assets
(
    asset_id            UInt64,
    owner               String,
    collection_name     String,
    template_id         Int32,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (asset_id)
ORDER BY (asset_id, owner, template_id, collection_name);

CREATE TABLE Collections
(
    collection_name     String,
    author              String,
    allow_notify        bool,
    authorized_accounts Array(String),
    notify_accounts     Array(String),
    market_fee          Float64,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (collection_name)
ORDER BY (collection_name, author, market_fee);

CREATE TABLE Schemas
(
    schema_name     String,
    collection_name String,
    format Nested (
        name        String,
        dtype       String,
    ),
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (schema_name)
ORDER BY (schema_name, collection_name);

CREATE TABLE Templates
(
    template_id     UInt64,
    schema_name     String,
    transferable    bool,
    burnable        bool,
    max_supply      UInt32,
    issued_supply   UInt32,
    collection_name String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (template_id)
ORDER BY (template_id, schema_name, collection_name);

CREATE TABLE Balances
(
    owner       String,
    quantities  Array(String),
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (owner)
ORDER BY (owner, quantities);

CREATE TABLE Offers
(
    offer_id                UInt64,
    sender                  String,
    recipient               String,
    sender_asset_ids        Array(UInt64),
    recipient_asset_ids     Array(UInt64),
    memo                    String,
    ram_payer               String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (offer_id)
ORDER BY (offer_id, sender, recipient);

CREATE TABLE Transfers
(
    collection_name    String,
    from               String,
    to                 String,
    asset_ids          Array(UInt64),
    memo               String,
)
ENGINE = MergeTree
PRIMARY KEY (collection_name)
ORDER BY (collection_name, from, to);