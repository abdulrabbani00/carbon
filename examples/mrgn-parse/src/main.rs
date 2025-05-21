use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    marginfi_v2_decoder_v0_1_2::{
        instructions::MarginfiInstruction as MarginfiV2DecoderV012Instruction, MarginfiDecoder,
        PROGRAM_ID as MARGINFI_PROGRAM_ID,
    },
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "kamino_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![MARGINFI_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![MARGINFI_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "marginfi_transaction_filter".to_string(),
        transaction_filter,
    );

    let yellowstone_grpc: YellowstoneGrpcGeyserClient = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    log::info!("Starting the MarginfiV2DecoderV012 instruction processor");
    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(5)
        .instruction(MarginfiDecoder, MarginfiV2DecoderV012InstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}
pub struct MarginfiV2DecoderV012InstructionProcessor;

#[async_trait]
impl Processor for MarginfiV2DecoderV012InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<MarginfiV2DecoderV012Instruction>,
        NestedInstructions,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            MarginfiV2DecoderV012Instruction::LendingAccountBorrow(data) => {
                log::info!(
                    "received the MarginfiV2DecoderV012 instruction, sig: {}, accounts len: {}",
                    signature,
                    accounts.len()
                );
            }
            _ => {
                log::info!(
                    "received the MarginfiV2DecoderV012 instruction, sig: {}, accounts len: {}",
                    signature,
                    accounts.len()
                );
            }
        };

        Ok(())
    }
}
