// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag = "1")]
    pub instructions: ::prost::alloc::vec::Vec<Meta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    #[prost(string, tag = "1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub block_time: i64,
    #[prost(uint64, tag = "3")]
    pub block_slot: u64,
    #[prost(string, tag = "4")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "5")]
    pub instruction_index: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub is_inner_instruction: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub inner_instruction_index: ::core::option::Option<u32>,
    #[prost(string, tag = "8")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub args: ::core::option::Option<FlatArg>,
    #[prost(map = "string, string", tag = "11")]
    pub input_accounts: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "12")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub outer_program: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatArg {
    /// CreateAmmConfig
    #[prost(uint32, optional, tag = "1")]
    pub index: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub trade_fee_rate: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub protocol_fee_rate: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub fund_fee_rate: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub create_pool_fee: ::core::option::Option<u64>,
    /// UpdateAmmConfig
    #[prost(uint32, optional, tag = "6")]
    pub param: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "7")]
    pub value: ::core::option::Option<u64>,
    /// UpdatePoolStatus
    #[prost(uint32, optional, tag = "8")]
    pub status: ::core::option::Option<u32>,
    /// CollectProtocolFee, CollectFundFee
    #[prost(string, optional, tag = "9")]
    pub amount0_requested: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub amount1_requested: ::core::option::Option<::prost::alloc::string::String>,
    /// Initialize
    #[prost(uint64, optional, tag = "11")]
    pub init_amount0: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "12")]
    pub init_amount1: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "13")]
    pub open_time: ::core::option::Option<u64>,
    /// Deposit
    #[prost(uint64, optional, tag = "14")]
    pub lp_token_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "15")]
    pub maximum_token0_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "16")]
    pub maximum_token1_amount: ::core::option::Option<u64>,
    /// Withdraw
    #[prost(uint64, optional, tag = "17")]
    pub minimum_token0_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "18")]
    pub minimum_token1_amount: ::core::option::Option<u64>,
    /// SwapBaseInput
    #[prost(uint64, optional, tag = "19")]
    pub amount_in: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "20")]
    pub minimum_amount_out: ::core::option::Option<u64>,
    /// SwapBaseOutput
    #[prost(uint64, optional, tag = "21")]
    pub max_amount_in: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "22")]
    pub amount_out: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    /// CreateAmmConfig accounts
    #[prost(message, optional, tag = "1")]
    pub owner: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "2")]
    pub amm_config: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "3")]
    pub system_program: ::core::option::Option<InputAccount>,
    /// UpdatePoolStatus accounts
    #[prost(message, optional, tag = "4")]
    pub authority: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "5")]
    pub pool_state: ::core::option::Option<InputAccount>,
    /// CollectProtocolFee and CollectFundFee accounts
    /// (reusing owner, authority, poolState, ammConfig)
    #[prost(message, optional, tag = "6")]
    pub token0_vault: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "7")]
    pub token1_vault: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "8")]
    pub vault0_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "9")]
    pub vault1_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "10")]
    pub recipient_token0_account: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "11")]
    pub recipient_token1_account: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "12")]
    pub token_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "13")]
    pub token_program2022: ::core::option::Option<InputAccount>,
    /// Initialize accounts
    /// (reusing some accounts from above)
    #[prost(message, optional, tag = "14")]
    pub creator: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "15")]
    pub token0_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "16")]
    pub token1_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "17")]
    pub lp_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "18")]
    pub creator_token0: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "19")]
    pub creator_token1: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "20")]
    pub creator_lp_token: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "21")]
    pub create_pool_fee: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "22")]
    pub observation_state: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "23")]
    pub token0_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "24")]
    pub token1_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "25")]
    pub associated_token_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "26")]
    pub rent: ::core::option::Option<InputAccount>,
    /// Deposit accounts
    /// (reusing several from above)
    #[prost(message, optional, tag = "27")]
    pub owner_lp_token: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "28")]
    pub token0_account: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "29")]
    pub token1_account: ::core::option::Option<InputAccount>,
    /// Withdraw accounts
    /// (reusing several from above)
    #[prost(message, optional, tag = "30")]
    pub memo_program: ::core::option::Option<InputAccount>,
    /// SwapBaseInput and SwapBaseOutput accounts
    /// (reusing several from above)
    #[prost(message, optional, tag = "31")]
    pub payer: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "32")]
    pub input_token_account: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "33")]
    pub output_token_account: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "34")]
    pub input_vault: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "35")]
    pub output_vault: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "36")]
    pub input_token_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "37")]
    pub output_token_program: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "38")]
    pub input_token_mint: ::core::option::Option<InputAccount>,
    #[prost(message, optional, tag = "39")]
    pub output_token_mint: ::core::option::Option<InputAccount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_signer: bool,
    #[prost(bool, tag = "3")]
    pub is_writable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPubKey {
    /// 32 bytes
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInt128 {
    /// Representing u128 as string
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InstructionArgs {
    #[prost(
        oneof = "instruction_args::InstructionArgs",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub instruction_args: ::core::option::Option<instruction_args::InstructionArgs>,
}
/// Nested message and enum types in `InstructionArgs`.
pub mod instruction_args {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum InstructionArgs {
        #[prost(message, tag = "1")]
        DefaultInstruction(super::DefaultInstructionLayout),
        #[prost(message, tag = "2")]
        CreateAmmConfig(super::PbCreateAmmConfigLayout),
        #[prost(message, tag = "3")]
        UpdateAmmConfig(super::PbUpdateAmmConfigLayout),
        #[prost(message, tag = "4")]
        UpdatePoolStatus(super::PbUpdatePoolStatusLayout),
        #[prost(message, tag = "5")]
        CollectProtocolFee(super::PbCollectProtocolFeeLayout),
        #[prost(message, tag = "6")]
        CollectFundFee(super::PbCollectFundFeeLayout),
        #[prost(message, tag = "7")]
        Initialize(super::PbInitializeLayout),
        #[prost(message, tag = "8")]
        Deposit(super::PbDepositLayout),
        #[prost(message, tag = "9")]
        Withdraw(super::PbWithdrawLayout),
        #[prost(message, tag = "10")]
        SwapBaseInput(super::PbSwapBaseInputLayout),
        #[prost(message, tag = "11")]
        SwapBaseOutput(super::PbSwapBaseOutputLayout),
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DefaultInstructionLayout {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbCreateAmmConfigLayout {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(uint64, tag = "2")]
    pub trade_fee_rate: u64,
    #[prost(uint64, tag = "3")]
    pub protocol_fee_rate: u64,
    #[prost(uint64, tag = "4")]
    pub fund_fee_rate: u64,
    #[prost(uint64, tag = "5")]
    pub create_pool_fee: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbUpdateAmmConfigLayout {
    #[prost(uint32, tag = "1")]
    pub param: u32,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbUpdatePoolStatusLayout {
    #[prost(uint32, tag = "1")]
    pub status: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbCollectProtocolFeeLayout {
    #[prost(uint64, tag = "1")]
    pub amount0_requested: u64,
    #[prost(uint64, tag = "2")]
    pub amount1_requested: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbCollectFundFeeLayout {
    #[prost(uint64, tag = "1")]
    pub amount0_requested: u64,
    #[prost(uint64, tag = "2")]
    pub amount1_requested: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbInitializeLayout {
    #[prost(uint64, tag = "1")]
    pub init_amount0: u64,
    #[prost(uint64, tag = "2")]
    pub init_amount1: u64,
    #[prost(uint64, tag = "3")]
    pub open_time: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbDepositLayout {
    #[prost(uint64, tag = "1")]
    pub lp_token_amount: u64,
    #[prost(uint64, tag = "2")]
    pub maximum_token0_amount: u64,
    #[prost(uint64, tag = "3")]
    pub maximum_token1_amount: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdrawLayout {
    #[prost(uint64, tag = "1")]
    pub lp_token_amount: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_token0_amount: u64,
    #[prost(uint64, tag = "3")]
    pub minimum_token1_amount: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbSwapBaseInputLayout {
    #[prost(uint64, tag = "1")]
    pub amount_in: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_amount_out: u64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbSwapBaseOutputLayout {
    #[prost(uint64, tag = "1")]
    pub max_amount_in: u64,
    #[prost(uint64, tag = "2")]
    pub amount_out: u64,
}
