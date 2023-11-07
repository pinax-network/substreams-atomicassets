// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnyEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvent {
    #[prost(oneof="any_event::Event", tags="1, 2, 3, 4, 5, 6, 7")]
    pub event: ::core::option::Option<any_event::Event>,
}
/// Nested message and enum types in `AnyEvent`.
pub mod any_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        AssetsTableItem(super::AssetsTableOperation),
        #[prost(message, tag="2")]
        TransferItem(super::TransferEvent),
        #[prost(message, tag="3")]
        SchemasTableItem(super::SchemasTableOperation),
        #[prost(message, tag="4")]
        CollectionsTableItem(super::CollectionsTableOperation),
        #[prost(message, tag="5")]
        TemplatesTableItem(super::TemplatesTableOperation),
        #[prost(message, tag="6")]
        BalancesTableItem(super::BalancesTableOperation),
        #[prost(message, tag="7")]
        OffersTableItem(super::OffersTableOperation),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="2")]
    pub db_operation: i32,
    #[prost(string, tag="3")]
    pub scope: ::prost::alloc::string::String,
    /// data payload
    #[prost(uint64, tag="4")]
    pub asset_id: u64,
    #[prost(string, tag="5")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(int32, tag="7")]
    pub template_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="3")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub memo: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Format {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dtype: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemasTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    /// data payload
    #[prost(string, tag="4")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub format: ::prost::alloc::vec::Vec<Format>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionsTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    /// data payload
    #[prost(string, tag="4")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub author: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub allow_notify: bool,
    #[prost(string, repeated, tag="7")]
    pub authorized_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub notify_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(double, tag="9")]
    pub market_fee: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplatesTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    /// data payload
    #[prost(int32, tag="4")]
    pub template_id: i32,
    #[prost(string, tag="5")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub transferable: bool,
    #[prost(bool, tag="7")]
    pub burnable: bool,
    #[prost(uint32, tag="8")]
    pub max_supply: u32,
    #[prost(uint32, tag="9")]
    pub issued_supply: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancesTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    /// data payload
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub quantities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    #[prost(uint64, tag="4")]
    pub offer_id: u64,
    #[prost(string, tag="5")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="7")]
    pub sender_asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="8")]
    pub recipient_asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub ram_payer: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
