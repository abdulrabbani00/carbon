



use super::MarginfiDecoder;
pub mod config_group_fee;
pub mod edit_global_fee_state;
pub mod edit_staked_settings;
pub mod init_global_fee_state;
pub mod init_staked_settings;
pub mod lending_account_borrow;
pub mod lending_account_close_balance;
pub mod lending_account_deposit;
pub mod lending_account_end_flashloan;
pub mod lending_account_liquidate;
pub mod lending_account_pulse_health;
pub mod lending_account_repay;
pub mod lending_account_settle_emissions;
pub mod lending_account_start_flashloan;
pub mod lending_account_withdraw;
pub mod lending_account_withdraw_emissions;
pub mod lending_account_withdraw_emissions_permissionless;
pub mod lending_pool_accrue_bank_interest;
pub mod lending_pool_add_bank;
pub mod lending_pool_add_bank_permissionless;
pub mod lending_pool_add_bank_with_seed;
pub mod lending_pool_collect_bank_fees;
pub mod lending_pool_configure_bank;
pub mod lending_pool_configure_bank_oracle;
pub mod lending_pool_handle_bankruptcy;
pub mod lending_pool_setup_emissions;
pub mod lending_pool_update_emissions_parameters;
pub mod lending_pool_withdraw_fees;
pub mod lending_pool_withdraw_insurance;
pub mod marginfi_account_close;
pub mod marginfi_account_initialize;
pub mod marginfi_account_update_emissions_destination_account;
pub mod marginfi_group_configure;
pub mod marginfi_group_initialize;
pub mod propagate_fee_state;
pub mod propagate_staked_settings;
pub mod set_account_flag;
pub mod set_new_account_authority;
pub mod unset_account_flag;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum MarginfiInstruction {
    ConfigGroupFee(config_group_fee::ConfigGroupFee),
    EditGlobalFeeState(edit_global_fee_state::EditGlobalFeeState),
    EditStakedSettings(edit_staked_settings::EditStakedSettings),
    InitGlobalFeeState(init_global_fee_state::InitGlobalFeeState),
    InitStakedSettings(init_staked_settings::InitStakedSettings),
    LendingAccountBorrow(lending_account_borrow::LendingAccountBorrow),
    LendingAccountCloseBalance(lending_account_close_balance::LendingAccountCloseBalance),
    LendingAccountDeposit(lending_account_deposit::LendingAccountDeposit),
    LendingAccountEndFlashloan(lending_account_end_flashloan::LendingAccountEndFlashloan),
    LendingAccountLiquidate(lending_account_liquidate::LendingAccountLiquidate),
    LendingAccountPulseHealth(lending_account_pulse_health::LendingAccountPulseHealth),
    LendingAccountRepay(lending_account_repay::LendingAccountRepay),
    LendingAccountSettleEmissions(lending_account_settle_emissions::LendingAccountSettleEmissions),
    LendingAccountStartFlashloan(lending_account_start_flashloan::LendingAccountStartFlashloan),
    LendingAccountWithdraw(lending_account_withdraw::LendingAccountWithdraw),
    LendingAccountWithdrawEmissions(lending_account_withdraw_emissions::LendingAccountWithdrawEmissions),
    LendingAccountWithdrawEmissionsPermissionless(lending_account_withdraw_emissions_permissionless::LendingAccountWithdrawEmissionsPermissionless),
    LendingPoolAccrueBankInterest(lending_pool_accrue_bank_interest::LendingPoolAccrueBankInterest),
    LendingPoolAddBank(lending_pool_add_bank::LendingPoolAddBank),
    LendingPoolAddBankPermissionless(lending_pool_add_bank_permissionless::LendingPoolAddBankPermissionless),
    LendingPoolAddBankWithSeed(lending_pool_add_bank_with_seed::LendingPoolAddBankWithSeed),
    LendingPoolCollectBankFees(lending_pool_collect_bank_fees::LendingPoolCollectBankFees),
    LendingPoolConfigureBank(lending_pool_configure_bank::LendingPoolConfigureBank),
    LendingPoolConfigureBankOracle(lending_pool_configure_bank_oracle::LendingPoolConfigureBankOracle),
    LendingPoolHandleBankruptcy(lending_pool_handle_bankruptcy::LendingPoolHandleBankruptcy),
    LendingPoolSetupEmissions(lending_pool_setup_emissions::LendingPoolSetupEmissions),
    LendingPoolUpdateEmissionsParameters(lending_pool_update_emissions_parameters::LendingPoolUpdateEmissionsParameters),
    LendingPoolWithdrawFees(lending_pool_withdraw_fees::LendingPoolWithdrawFees),
    LendingPoolWithdrawInsurance(lending_pool_withdraw_insurance::LendingPoolWithdrawInsurance),
    MarginfiAccountClose(marginfi_account_close::MarginfiAccountClose),
    MarginfiAccountInitialize(marginfi_account_initialize::MarginfiAccountInitialize),
    MarginfiAccountUpdateEmissionsDestinationAccount(marginfi_account_update_emissions_destination_account::MarginfiAccountUpdateEmissionsDestinationAccount),
    MarginfiGroupConfigure(marginfi_group_configure::MarginfiGroupConfigure),
    MarginfiGroupInitialize(marginfi_group_initialize::MarginfiGroupInitialize),
    PropagateFeeState(propagate_fee_state::PropagateFeeState),
    PropagateStakedSettings(propagate_staked_settings::PropagateStakedSettings),
    SetAccountFlag(set_account_flag::SetAccountFlag),
    SetNewAccountAuthority(set_new_account_authority::SetNewAccountAuthority),
    UnsetAccountFlag(unset_account_flag::UnsetAccountFlag),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MarginfiDecoder {
    type InstructionType = MarginfiInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MarginfiInstruction::ConfigGroupFee => config_group_fee::ConfigGroupFee,
            MarginfiInstruction::EditGlobalFeeState => edit_global_fee_state::EditGlobalFeeState,
            MarginfiInstruction::EditStakedSettings => edit_staked_settings::EditStakedSettings,
            MarginfiInstruction::InitGlobalFeeState => init_global_fee_state::InitGlobalFeeState,
            MarginfiInstruction::InitStakedSettings => init_staked_settings::InitStakedSettings,
            MarginfiInstruction::LendingAccountBorrow => lending_account_borrow::LendingAccountBorrow,
            MarginfiInstruction::LendingAccountCloseBalance => lending_account_close_balance::LendingAccountCloseBalance,
            MarginfiInstruction::LendingAccountDeposit => lending_account_deposit::LendingAccountDeposit,
            MarginfiInstruction::LendingAccountEndFlashloan => lending_account_end_flashloan::LendingAccountEndFlashloan,
            MarginfiInstruction::LendingAccountLiquidate => lending_account_liquidate::LendingAccountLiquidate,
            MarginfiInstruction::LendingAccountPulseHealth => lending_account_pulse_health::LendingAccountPulseHealth,
            MarginfiInstruction::LendingAccountRepay => lending_account_repay::LendingAccountRepay,
            MarginfiInstruction::LendingAccountSettleEmissions => lending_account_settle_emissions::LendingAccountSettleEmissions,
            MarginfiInstruction::LendingAccountStartFlashloan => lending_account_start_flashloan::LendingAccountStartFlashloan,
            MarginfiInstruction::LendingAccountWithdraw => lending_account_withdraw::LendingAccountWithdraw,
            MarginfiInstruction::LendingAccountWithdrawEmissions => lending_account_withdraw_emissions::LendingAccountWithdrawEmissions,
            MarginfiInstruction::LendingAccountWithdrawEmissionsPermissionless => lending_account_withdraw_emissions_permissionless::LendingAccountWithdrawEmissionsPermissionless,
            MarginfiInstruction::LendingPoolAccrueBankInterest => lending_pool_accrue_bank_interest::LendingPoolAccrueBankInterest,
            MarginfiInstruction::LendingPoolAddBank => lending_pool_add_bank::LendingPoolAddBank,
            MarginfiInstruction::LendingPoolAddBankPermissionless => lending_pool_add_bank_permissionless::LendingPoolAddBankPermissionless,
            MarginfiInstruction::LendingPoolAddBankWithSeed => lending_pool_add_bank_with_seed::LendingPoolAddBankWithSeed,
            MarginfiInstruction::LendingPoolCollectBankFees => lending_pool_collect_bank_fees::LendingPoolCollectBankFees,
            MarginfiInstruction::LendingPoolConfigureBank => lending_pool_configure_bank::LendingPoolConfigureBank,
            MarginfiInstruction::LendingPoolConfigureBankOracle => lending_pool_configure_bank_oracle::LendingPoolConfigureBankOracle,
            MarginfiInstruction::LendingPoolHandleBankruptcy => lending_pool_handle_bankruptcy::LendingPoolHandleBankruptcy,
            MarginfiInstruction::LendingPoolSetupEmissions => lending_pool_setup_emissions::LendingPoolSetupEmissions,
            MarginfiInstruction::LendingPoolUpdateEmissionsParameters => lending_pool_update_emissions_parameters::LendingPoolUpdateEmissionsParameters,
            MarginfiInstruction::LendingPoolWithdrawFees => lending_pool_withdraw_fees::LendingPoolWithdrawFees,
            MarginfiInstruction::LendingPoolWithdrawInsurance => lending_pool_withdraw_insurance::LendingPoolWithdrawInsurance,
            MarginfiInstruction::MarginfiAccountClose => marginfi_account_close::MarginfiAccountClose,
            MarginfiInstruction::MarginfiAccountInitialize => marginfi_account_initialize::MarginfiAccountInitialize,
            MarginfiInstruction::MarginfiAccountUpdateEmissionsDestinationAccount => marginfi_account_update_emissions_destination_account::MarginfiAccountUpdateEmissionsDestinationAccount,
            MarginfiInstruction::MarginfiGroupConfigure => marginfi_group_configure::MarginfiGroupConfigure,
            MarginfiInstruction::MarginfiGroupInitialize => marginfi_group_initialize::MarginfiGroupInitialize,
            MarginfiInstruction::PropagateFeeState => propagate_fee_state::PropagateFeeState,
            MarginfiInstruction::PropagateStakedSettings => propagate_staked_settings::PropagateStakedSettings,
            MarginfiInstruction::SetAccountFlag => set_account_flag::SetAccountFlag,
            MarginfiInstruction::SetNewAccountAuthority => set_new_account_authority::SetNewAccountAuthority,
            MarginfiInstruction::UnsetAccountFlag => unset_account_flag::UnsetAccountFlag,
        )
    }
}