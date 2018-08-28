use bitcoin_support;
use common_types::{
    ledger::{bitcoin::Bitcoin, ethereum::Ethereum},
    TradingSymbol,
};
use ethereum_support;
use exchange_api_client::{
    client::OrderResponseBody, ApiClient, OfferResponseBody, OrderRequestBody,
};
use reqwest;
use swaps::TradeId;
use uuid::Uuid;

#[allow(dead_code)]
pub struct FakeApiClient;

impl FakeApiClient {
    pub fn new() -> Self {
        FakeApiClient {}
    }
}

impl ApiClient for FakeApiClient {
    fn create_buy_offer(
        &self,
        symbol: TradingSymbol,
        _amount: f64,
    ) -> Result<OfferResponseBody<Ethereum, Bitcoin>, reqwest::Error> {
        let offer = OfferResponseBody {
            uid: TradeId::from(Uuid::new_v4()),
            symbol,
            rate: 0.42,
            sell_amount: bitcoin_support::BitcoinQuantity::from_bitcoin(24.0),
            buy_amount: ethereum_support::EthereumQuantity::from_eth(1.0),
        };
        Ok(offer)
    }

    fn create_buy_order(
        &self,
        _symbol: TradingSymbol,
        _uid: TradeId,
        _trade_request: &OrderRequestBody,
    ) -> Result<OrderResponseBody, reqwest::Error> {
        let accept = OrderResponseBody {
            exchange_refund_address: String::from("34b19d15e793883d840c563d7dbc8a6723465146"),
            exchange_contract_time_lock: 43200,
            exchange_success_address: String::from("bcrt1qcqslz7lfn34dl096t5uwurff9spen5h4v2pmap"),
        };

        Ok(accept)
    }

    fn create_sell_offer(
        &self,
        symbol: TradingSymbol,
        _amount: f64,
    ) -> Result<OfferResponseBody<Bitcoin, Ethereum>, reqwest::Error> {
        let offer = OfferResponseBody {
            uid: TradeId::from(Uuid::new_v4()),
            symbol,
            rate: 0.24,
            sell_amount: ethereum_support::EthereumQuantity::from_eth(1.0),
            buy_amount: bitcoin_support::BitcoinQuantity::from_bitcoin(24.0),
        };
        Ok(offer)
    }

    fn create_sell_order(
        &self,
        _symbol: TradingSymbol,
        _uid: TradeId,
        _trade_request: &OrderRequestBody,
    ) -> Result<OrderResponseBody, reqwest::Error> {
        let accept = OrderResponseBody {
            exchange_refund_address: String::from("bcrt1qcqslz7lfn34dl096t5uwurff9spen5h4v2pmap"),
            exchange_contract_time_lock: 43200,
            exchange_success_address: String::from("34b19d15e793883d840c563d7dbc8a6723465146"),
        };

        Ok(accept)
    }
}
