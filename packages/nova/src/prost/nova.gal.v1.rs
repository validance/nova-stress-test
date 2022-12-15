/// MsgDeposit defines user who deposit and amount of coins.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {
    #[prost(string, tag = "1")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub deposited_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegate {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(uint64, tag = "4")]
    pub timeout_timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUndelegate {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(uint64, tag = "4")]
    pub timeout_timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUndelegateResponse {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub total_burn_asset: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "3")]
    pub total_undelegate_asset: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// MsgUnStaking defines user who want to un-stake his/her asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPendingUndelegate {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPendingUndelegateResponse {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub burn_asset: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "5")]
    pub undelegate_asset: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// MsgWithdraw defines user who withdraw and amount of coins.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdrawer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {
    #[prost(string, tag = "1")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdraw_amount: ::prost::alloc::string::String,
}
/// MsgClaim defines claim msg used when user want to claim their st token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimSnAsset {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub claimer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimSnAssetResponse {
    #[prost(string, tag = "1")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub minted: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIcaWithdraw {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ica_transfer_port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ica_transfer_channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub chain_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "6")]
    pub version: u64,
    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIcaWithdrawResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbcTrace {
    #[prost(uint64, tag = "1")]
    pub version: u64,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(uint64, tag = "3")]
    pub state: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionState {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub current_version: u64,
    #[prost(map = "uint64, message", tag = "3")]
    pub record: ::std::collections::HashMap<u64, IbcTrace>,
}
/// Params defines the parameters for the gal module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositRecord {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DepositRecordContent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositRecordContent {
    #[prost(string, tag = "1")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "3")]
    pub state: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegateRecord {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(map = "uint64, message", tag = "3")]
    pub records: ::std::collections::HashMap<u64, DelegateRecordContent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegateRecordContent {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "2")]
    pub state: i64,
    #[prost(uint64, tag = "3")]
    pub oracle_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndelegateRecord {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<UndelegateRecordContent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndelegateRecordContent {
    #[prost(string, tag = "1")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub sn_asset_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(string, tag = "3")]
    pub withdraw_amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub state: i64,
    #[prost(uint64, tag = "5")]
    pub oracle_version: u64,
    #[prost(uint64, tag = "6")]
    pub undelegate_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRecord {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(map = "uint64, message", tag = "3")]
    pub records: ::std::collections::HashMap<u64, WithdrawRecordContent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRecordContent {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub unstaking_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(int64, tag = "3")]
    pub state: i64,
    #[prost(int64, tag = "4")]
    pub oracle_version: i64,
    #[prost(uint64, tag = "5")]
    pub withdraw_version: u64,
    #[prost(message, optional, tag = "6")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryClaimableAmountRequest is the request type for the Query/ClaimableAmount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableAmountRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryClaimableAmountResponse is the response type for the Query/ClaimableAmount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimableAmountResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryPendingWithdrawalsRequest is the request type for the Query/PendingWithdrawals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingWithdrawalsRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryPendingWithdrawalsResponse is the response type for the Query/PendingWithdrawals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingWithdrawalsResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryDepositAmountRequest is the request type for the Query/DepositAmount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositAmountRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryDepositAmountResponse is the response type for the Query/DepositAmount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositAmountResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryActiveWithdrawalsRequest is the request type for the Query/ActiveWithdrawals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveWithdrawalsRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryActiveWithdrawalsResponse is the response type for the Query/ActiveWithdrawals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveWithdrawalsResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRecordRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub deposit_record: ::core::option::Option<DepositRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateRecordRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub delegate_record: ::core::option::Option<DelegateRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUndelegateRecordRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUndelegateRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub undelegate_record: ::core::option::Option<UndelegateRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRecordRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub withdraw_record: ::core::option::Option<WithdrawRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateSnAssetRequest {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateSnAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub version_info: ::core::option::Option<IbcTrace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUndelegateVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUndelegateVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub version_info: ::core::option::Option<IbcTrace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub version_info: ::core::option::Option<IbcTrace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentDelegateVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentDelegateVersionResponse {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentUndelegateVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentUndelegateVersionResponse {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentWithdrawVersion {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentWithdrawVersionResponse {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSnAssetSupply {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSnAssetSupplyResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EventDeposit is emitted when user deposit their asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDeposit {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub deposit_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// EventDelegate is emitted when service bot delegates accumulated asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDelegate {
    #[prost(string, tag = "1")]
    pub host_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub delegated_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// EventPendingUndelegate is emitted when pending undelegate message is submitted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPendingUndelegate {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub burned_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "5")]
    pub undelegated_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// EventUndelegate is emitted when remote undelegate message is submitted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUndelegate {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub burned_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "3")]
    pub undelegated_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// EventWithdraw is emitted when withdraw message is submitted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdraw {
    #[prost(string, tag = "1")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdrawer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub withdrawn_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// EventClaimSnToken is emitted when snAsset is minted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventClaimSnToken {
    #[prost(string, tag = "1")]
    pub claimer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub claimed_token: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(uint64, tag = "3")]
    pub oracle_version: u64,
}
/// EventIcaWithdraw is emitted when ica withdraw action is success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventIcaWithdraw {
    #[prost(string, tag = "1")]
    pub host_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub withdrawn_token: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// GenesisState defines the gal module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub deposit_record: ::prost::alloc::vec::Vec<DepositRecord>,
    #[prost(message, repeated, tag = "3")]
    pub delegate_record: ::prost::alloc::vec::Vec<DelegateRecord>,
    #[prost(message, repeated, tag = "4")]
    pub undelegate_record: ::prost::alloc::vec::Vec<UndelegateRecord>,
    #[prost(message, repeated, tag = "5")]
    pub withdraw_record: ::prost::alloc::vec::Vec<WithdrawRecord>,
    #[prost(message, repeated, tag = "6")]
    pub record_info: ::prost::alloc::vec::Vec<RecordInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordInfo {
    #[prost(message, optional, tag = "1")]
    pub delegate_trace: ::core::option::Option<VersionState>,
    #[prost(message, optional, tag = "2")]
    pub undelegate_trace: ::core::option::Option<VersionState>,
    #[prost(message, optional, tag = "3")]
    pub withdraw_trace: ::core::option::Option<VersionState>,
}
