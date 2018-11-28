use bitcoin_support::{BitcoinQuantity, OutPoint};
use bitcoin_witness::PrimedInput;
use ethereum_support::{Bytes, Erc20Quantity, EtherQuantity};
use swap_protocols::{
    ledger::{Bitcoin, Ethereum},
    rfc003::{
        actions::{Accept, Action, Decline, StateActions},
        bitcoin,
        ethereum::{self, Erc20Htlc, Htlc},
        roles::Bob,
        secret::Secret,
        state_machine::*,
    },
};

impl OngoingSwap<Bob<Bitcoin, Ethereum, BitcoinQuantity, Erc20Quantity>> {
    pub fn deploy_action(&self) -> ethereum::ContractDeploy {
        let htlc = Erc20Htlc::from(self.beta_htlc_params());
        ethereum::ContractDeploy {
            data: htlc.compile_to_hex().into(),
            value: EtherQuantity::zero(),
            gas_limit: 420_000.into(), //TODO: Calculate properly
        }
    }

    pub fn refund_action(
        &self,
        beta_htlc_location: ethereum_support::Address,
    ) -> ethereum::SendTransaction {
        ethereum::SendTransaction {
            to: beta_htlc_location,
            data: Bytes::default(),
            gas_limit: 42_000.into(), //TODO: Calculate properly
            value: EtherQuantity::zero(),
        }
    }

    pub fn fund_action(
        &self,
        beta_htlc_location: ethereum_support::Address,
    ) -> ethereum::SendTransaction {
        let htlc = Erc20Htlc::from(self.beta_htlc_params());
        ethereum::SendTransaction {
            to: self.beta_asset.token_contract(),
            data: htlc.funding_tx_payload(beta_htlc_location),
            gas_limit: 42_000.into(), //TODO: Calculate properly
            value: EtherQuantity::zero(),
        }
    }

    pub fn redeem_action(
        &self,
        beta_htlc_location: OutPoint,
        secret: Secret,
    ) -> bitcoin::SpendOutput {
        bitcoin::SpendOutput {
            output: PrimedInput::new(
                beta_htlc_location,
                self.alpha_asset,
                bitcoin::Htlc::from(self.alpha_htlc_params())
                    .unlock_with_secret(self.alpha_ledger_success_identity, &secret),
            ),
        }
    }
}

impl StateActions for SwapStates<Bob<Bitcoin, Ethereum, BitcoinQuantity, Erc20Quantity>> {
    type Accept = Accept;
    type Decline = Decline;
    type Deploy = ethereum::ContractDeploy;
    type Fund = ethereum::SendTransaction;
    type Redeem = bitcoin::SpendOutput;
    type Refund = ethereum::SendTransaction;

    fn actions(
        &self,
    ) -> Vec<
        Action<
            Accept,
            Decline,
            ethereum::ContractDeploy,
            ethereum::SendTransaction,
            bitcoin::SpendOutput,
            ethereum::SendTransaction,
        >,
    > {
        use self::SwapStates as SS;
        match *self {
            SS::Start { .. } => vec![Action::Accept(Accept), Action::Decline(Decline)],
            SS::AlphaFunded(AlphaFunded { ref swap, .. }) => {
                vec![Action::Deploy(swap.deploy_action())]
            }
            SS::AlphaFundedBetaDeployed(AlphaFundedBetaDeployed {
                ref swap,
                ref beta_htlc_location,
                ..
            }) => vec![Action::Fund(swap.fund_action(*beta_htlc_location))],
            SS::BothFunded(BothFunded {
                ref beta_htlc_location,
                ref swap,
                ..
            }) => vec![Action::Refund(swap.refund_action(*beta_htlc_location))],
            SS::AlphaFundedBetaRefunded { .. } => vec![],
            SS::AlphaRedeemedBetaFunded(AlphaRedeemedBetaFunded {
                ref beta_htlc_location,
                ref swap,
                ..
            }) => vec![Action::Refund(swap.refund_action(*beta_htlc_location))],
            SS::AlphaRefundedBetaFunded(AlphaRefundedBetaFunded {
                ref beta_htlc_location,
                ref swap,
                ..
            }) => vec![Action::Refund(swap.refund_action(*beta_htlc_location))],
            SS::AlphaFundedBetaRedeemed(AlphaFundedBetaRedeemed {
                ref swap,
                ref alpha_htlc_location,
                ref secret,
                ..
            }) => vec![Action::Redeem(
                swap.redeem_action(*alpha_htlc_location, *secret),
            )],
            _ => vec![],
        }
    }
}
