// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsTableOperations {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AssetsTableOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsTableOperation {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// database operation
    #[prost(int32, tag="3")]
    pub db_operation: i32,
    #[prost(string, tag="12")]
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
// @@protoc_insertion_point(module)
