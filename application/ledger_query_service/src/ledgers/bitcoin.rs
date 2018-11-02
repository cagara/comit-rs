use bitcoin_support::{
    serialize::BitcoinHash, Address, MinedBlock as BitcoinBlock, SpendsTo,
    Transaction as BitcoinTransaction,
};
use block_processor::{Block, Query, QueryMatchResult, Transaction};
use route_factory::QueryType;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BitcoinTransactionQuery {
    pub to_address: Option<Address>,
    #[serde(default = "default_confirmations")]
    confirmations_needed: u32,
}

impl QueryType for BitcoinTransactionQuery {
    fn route() -> &'static str {
        "transactions"
    }
}

fn default_confirmations() -> u32 {
    1
}

impl Query<BitcoinTransaction> for BitcoinTransactionQuery {
    fn matches(&self, transaction: &BitcoinTransaction) -> QueryMatchResult {
        match self.to_address {
            Some(ref address) => {
                if transaction.spends_to(address) {
                    QueryMatchResult::yes_with_confirmations(self.confirmations_needed)
                } else {
                    QueryMatchResult::no()
                }
            }
            None => {
                warn!("to_address not sent, no parameters to compare the transaction");
                QueryMatchResult::no()
            }
        }
    }
    fn is_empty(&self) -> bool {
        self.to_address.is_none()
    }
}

impl Transaction for BitcoinTransaction {
    fn transaction_id(&self) -> String {
        self.txid().to_string()
    }
}

impl Block for BitcoinBlock {
    type Transaction = BitcoinTransaction;

    fn blockhash(&self) -> String {
        format!("{:x}", self.as_ref().header.bitcoin_hash())
    }
    fn prev_blockhash(&self) -> String {
        format!("{:x}", self.as_ref().header.prev_blockhash)
    }
    fn transactions(&self) -> &[BitcoinTransaction] {
        self.as_ref().txdata.as_slice()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BitcoinBlockQuery {
    pub min_height: Option<u32>,
}

impl QueryType for BitcoinBlockQuery {
    fn route() -> &'static str {
        "blocks"
    }
}

impl Query<BitcoinBlock> for BitcoinBlockQuery {
    fn matches(&self, block: &BitcoinBlock) -> QueryMatchResult {
        match self.min_height {
            Some(ref height) => {
                if *height <= block.height {
                    QueryMatchResult::yes()
                } else {
                    QueryMatchResult::no()
                }
            }
            None => {
                warn!("min_height not set, nothing to compare");
                QueryMatchResult::no()
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.min_height.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin_support::{Block, BlockHeader, MinedBlock, Sha256dHash};
    use spectral::prelude::*;

    #[test]
    fn given_query_min_height_then_lesser_block_does_not_match() {
        let block_header = BlockHeader {
            version: 1,
            prev_blockhash: Sha256dHash::default(),
            merkle_root: Sha256dHash::default(),
            time: 0,
            bits: 1,
            nonce: 0,
        };

        let block = MinedBlock::new(
            Block {
                header: block_header,
                txdata: vec![],
            },
            40,
        );

        let query = BitcoinBlockQuery {
            min_height: Some(42),
        };

        assert_that(&query.matches(&block)).is_equal_to(QueryMatchResult::no());
    }

    #[test]
    fn given_query_min_height_then_exact_block_matches() {
        let block_header = BlockHeader {
            version: 1,
            prev_blockhash: Sha256dHash::default(),
            merkle_root: Sha256dHash::default(),
            time: 0,
            bits: 1,
            nonce: 0,
        };

        let block = MinedBlock::new(
            Block {
                header: block_header,
                txdata: vec![],
            },
            42,
        );

        let query = BitcoinBlockQuery {
            min_height: Some(42),
        };

        assert_that(&query.matches(&block)).is_equal_to(QueryMatchResult::yes());
    }

    #[test]
    fn given_query_min_height_then_greater_block_matches() {
        let block_header = BlockHeader {
            version: 1,
            prev_blockhash: Sha256dHash::default(),
            merkle_root: Sha256dHash::default(),
            time: 0,
            bits: 1,
            nonce: 0,
        };

        let block = MinedBlock::new(
            Block {
                header: block_header,
                txdata: vec![],
            },
            45,
        );

        let query = BitcoinBlockQuery {
            min_height: Some(42),
        };

        assert_that(&query.matches(&block)).is_equal_to(QueryMatchResult::yes());
    }

}