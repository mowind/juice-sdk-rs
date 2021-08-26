use crate::{
    helpers::{self, CallFuture},
    types::{
        Address, Block, BlockHeader, BlockNumber, Bytes, CallRequest, Index, SyncState,
        Transaction, TransactionReceipt, TransactionRequest, H256, U256, U64,
    },
    Transport,
};

/// Juice chain's client, similar to Go version's client.
#[derive(Debug, Clone)]
pub struct Client<T: Transport> {
    transport: T,
    is_http: bool,
}

impl<T: Transport> Client<T> {
    /// Create a new client.
    pub fn new(transport: T, is_http: bool) -> Self {
        Self { transport, is_http }
    }

    /// Get the current block number from chain.
    pub fn block_number(&self, ledger: String) -> CallFuture<U64, T::Out> {
        let ledger = helpers::serialize(&ledger);
        CallFuture::new(self.transport.execute("juice_blockNumber", vec![ledger]))
    }

    /// Returns the block details with the given hash.
    pub fn block_by_hash(
        &self,
        ledger: String,
        hash: H256,
    ) -> CallFuture<Option<Block<Transaction>>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let included_txs = helpers::serialize(&true);
        let hash = helpers::serialize(&hash);

        CallFuture::new(
            self.transport
                .execute("juice_getBlockByHash", vec![ledger, hash, included_txs]),
        )
    }

    /// Returns the block details with the given number.
    pub fn block_by_number(
        &self,
        ledger: String,
        number: BlockNumber,
    ) -> CallFuture<Option<Block<Transaction>>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let included_txs = helpers::serialize(&true);
        let number = helpers::serialize(&number);

        CallFuture::new(
            self.transport
                .execute("juice_getBlockByNumber", vec![ledger, number, included_txs]),
        )
    }

    /// Returns the block header with the given hash.
    pub fn header_by_hash(
        &self,
        ledger: String,
        hash: H256,
    ) -> CallFuture<Option<BlockHeader>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let excluded_txs = helpers::serialize(&false);
        let hash = helpers::serialize(&hash);

        CallFuture::new(
            self.transport
                .execute("juice_getBlockByHash", vec![ledger, hash, excluded_txs]),
        )
    }

    /// Returns a block header from the current canonical chain.
    pub fn header_by_number(
        &self,
        ledger: String,
        number: BlockNumber,
    ) -> CallFuture<Option<BlockHeader>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let excluded_txs = helpers::serialize(&false);
        let num = helpers::serialize(&number);

        CallFuture::new(
            self.transport
                .execute("juice_getBlockByNumber", vec![ledger, num, excluded_txs]),
        )
    }

    /// Returns the transation with the given hash.
    pub fn transaction_by_hash(
        &self,
        ledger: String,
        hash: H256,
    ) -> CallFuture<Option<Transaction>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let hash = helpers::serialize(&hash);

        CallFuture::new(
            self.transport
                .execute("juice_getTransactionByHash", vec![ledger, hash]),
        )
    }

    /// Returns the total number of transactions in the given block.
    pub fn transaction_count(&self, ledger: String, block_hash: H256) -> CallFuture<u32, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let block_hash = helpers::serialize(&block_hash);

        CallFuture::new(self.transport.execute(
            "juice_getBlockTransactionCountByHash",
            vec![ledger, block_hash],
        ))
    }

    /// Returns a single transaction at index in the given block.
    pub fn transaction_in_block(
        &self,
        ledger: String,
        block_hash: H256,
        index: Index,
    ) -> CallFuture<Option<Transaction>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let block_hash = helpers::serialize(&block_hash);
        let index = helpers::serialize(&index);

        CallFuture::new(self.transport.execute(
            "juice_getTransactionByBlockHashAndIndex",
            vec![ledger, block_hash, index],
        ))
    }

    /// Returns the receipt of a transaction by transaction hash.
    pub fn transaction_receipt(
        &self,
        ledger: String,
        tx_hash: H256,
    ) -> CallFuture<Option<TransactionReceipt>, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let tx_hash = helpers::serialize(&tx_hash);

        CallFuture::new(
            self.transport
                .execute("juice_getTransactionReceipt", vec![ledger, tx_hash]),
        )
    }

    /// Retrieves the current progress of the sync algorithm.
    pub fn sync_progress(&self, ledger: String) -> CallFuture<SyncState, T::Out> {
        let ledger = helpers::serialize(&ledger);
        CallFuture::new(self.transport.execute("juice_syncing", vec![ledger]))
    }

    /// Returns the VON balance of the given account.
    pub fn balance_at(
        &self,
        ledger: String,
        account: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<U256, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let account = helpers::serialize(&account);
        let number = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("juice_getBalance", vec![ledger, account, number]),
        )
    }

    /// Returns the value of key in the contract storage of the given account.
    pub fn storage_at(
        &self,
        ledger: String,
        account: Address,
        key: H256,
        block: Option<BlockNumber>,
    ) -> CallFuture<Bytes, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let account = helpers::serialize(&account);
        let key = helpers::serialize(&key);
        let number = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("juice_getStorageAt", vec![ledger, account, key, number]),
        )
    }

    /// Returns the contract code of the given account.
    pub fn code_at(
        &self,
        ledger: String,
        account: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<Bytes, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let account = helpers::serialize(&account);
        let number = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("juice_getCode", vec![ledger, account, number]),
        )
    }

    /// Returns the account nonce of the given account.
    pub fn nonce_at(
        &self,
        ledger: String,
        account: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<U64, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let account = helpers::serialize(&account);
        let number = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("juice_getTransactionCount", vec![ledger, account, number]),
        )
    }

    /// Contract calling
    pub fn call(
        &self,
        ledger: String,
        req: CallRequest,
        number: BlockNumber,
    ) -> CallFuture<Bytes, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let req = helpers::serialize(&req);
        let number = helpers::serialize(&number);

        CallFuture::new(
            self.transport
                .execute("juice_call", vec![ledger, req, number]),
        )
    }

    /// Retrieves the currently suggested gas price to allow a timely
    /// execution of a transaction.
    pub fn suggest_gas_price(&self, ledger: String) -> CallFuture<U256, T::Out> {
        let ledger = helpers::serialize(&ledger);
        CallFuture::new(self.transport.execute("juice_gasPrice", vec![ledger]))
    }

    /// Tries to estimate the gas needed to execute a specific transaction based on
    /// the current pending state of the backend blockchain. There is no guarantee
    /// That this the true gas limit requirement as other transaction may be added
    /// or removed by miners, but it should provide a basis for setting a reasonable
    /// default.
    pub fn estimate_gas(&self, ledger: String, req: CallRequest) -> CallFuture<U64, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let req = helpers::serialize(&req);
        CallFuture::new(
            self.transport
                .execute("juice_estimateGas", vec![ledger, req]),
        )
    }

    /// Injects a signed transaction into the pending pool for execution.
    ///
    /// If the transaction was a contract creation use the TransactionReceipt
    /// method to get the contract address after the transaction has been mined.
    pub fn send_transaction(
        &self,
        ledger: String,
        tx: TransactionRequest,
    ) -> CallFuture<H256, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let tx = helpers::serialize(&tx);
        CallFuture::new(
            self.transport
                .execute("juice_sendTransaction", vec![ledger, tx]),
        )
    }

    /// Injects a signed transaction into the pending pool for execution.
    pub fn send_raw_transaction(&self, ledger: String, rlp: Bytes) -> CallFuture<H256, T::Out> {
        let ledger = helpers::serialize(&ledger);
        let rlp = helpers::serialize(&rlp);
        CallFuture::new(
            self.transport
                .execute("juice_sendRawTransaction", vec![ledger, rlp]),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::rpc::Value;
    use crate::types::{
        Address, Block, BlockId, BlockNumber, Bytes, CallRequest, Log, SyncState, Transaction,
        TransactionId, TransactionReceipt, TransactionRequest, H256, H520, H64,
    };

    // response for RPC juice_getBlockByHash/juice_getBlockByNumber
    const EXAMPLE_BLOCK: &'static str = r#"{
        "extraData": "0x00000000000000000000000000000000000000000000000000000000000000008e4ffcc4c25d36a28e18db26fba22f7b76304c07b58037931c77ea98f24fccd175a60d6a36a9f81bc363b4d605963c6c3d8eaebdba86b87a2c4080d914d1a2d601",
        "gasLimit": "0x111",
        "gasUsed": "0x0",
        "hash": "0xddb59c26e1d77e82276c2a2d5eec61cae3d85bcdc20106f4e71f3555bbeebb6a",
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "miner": "juc1683skg34pc9cz6ks6whf7fcg3pt7nj3sezr770",
        "nonce": "0x02fb3522c488b843956c31b1def2f8ef17d4c12f8a669b29aca8ac866d74a1be4417e589c27fe4425f4b67ed174362636916bd48535ad853d85bc8ce8b0c9fd8afc7ebf003a3221c4aafeb4171bde8944d",
        "number": "0x233",
        "parentHash": "0xfcf0f581f818d1ab4fccd54439e630d81c4599f0782b0fc1d1ca1459f1ce348d",
        "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
        "size": 735,
        "stateRoot": "0x8ef44b33932f70b82c5ba4fea063dca310e53d60c059bb13ed319df66be292ca",
        "timestamp": "0x1111",
        "transactions": [],
        "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
    }"#;

    use super::Client;

    rpc_test!(
        Client:block_number, String::from("sys") => "juice_blockNumber", vec![r#""sys""#];
        Value::String("0x123".into()) => 0x123
    );

    rpc_test!(
        Client:balance_at,String::from("sys"), Address::from("lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww"), None
            =>
            "juice_getBalance", vec![r#""sys""#, r#""lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww""#, r#""latest""#];
        Value::String("0x123".into()) => 0x123
    );

    rpc_test!(
        Client:nonce_at, String::from("sys"), Address::from("lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww"), None
            => "juice_getTransactionCount", vec![r#""sys""#, r#""lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww""#, r#""latest""#];
        Value::String("0x123".into()) => 0x123
    );

    rpc_test!(
        Client:suggest_gas_price, String::from("sys") => "juice_gasPrice", vec![r#""sys""#];
        Value::String("0x123".into()) => 0x123
    );

    rpc_test!(
        Client:estimate_gas, String::from("sys"), CallRequest{
            from: None, to: Some(Address::from("lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww")),
            gas: None, gas_price: None,
            value: Some(0x1.into()), data: None,
        } =>
            "juice_estimateGas", vec![r#""sys""#, r#"{"to":"lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww","value":"0x1"}"#];
        Value::String("0x123".into()) => 0x123
    );

    rpc_test!(
        Client:storage_at, String::from("sys"), Address::from("lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww"), H256::from_low_u64_be(0x123), None
            => "juice_getStorageAt", vec![
                r#""sys""#,
                r#""lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww""#,
                r#""0x0000000000000000000000000000000000000000000000000000000000000123""#,
                r#""latest""#
            ];
        Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => H256::from_low_u64_be(0x123).to_fixed_bytes()
    );

    rpc_test!(
        Client:code_at, String::from("sys"), Address::from("lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww"),  None
            => "juice_getCode", vec![
                r#""sys""#,
                r#""lax18qg084alcnuv2jjdx4u68vw4ve8lffm0ncrzww""#,
                r#""latest""#
            ];
        Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => H256::from_low_u64_be(0x123).to_fixed_bytes()
    );

    rpc_test!(
        Client:block_by_number, String::from("sys"), BlockNumber::Pending
            => "juice_getBlockByNumber", vec![r#""sys""#, r#""pending""#, r#"true"#];
        ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
            => Some(::serde_json::from_str::<Block<Transaction>>(EXAMPLE_BLOCK).unwrap())
    );

    rpc_test!(
        Client:block_by_hash, String::from("sys"),H256::from_low_u64_be(0x123)
            => "juice_getBlockByHash", vec![r#""sys""#, r#""0x0000000000000000000000000000000000000000000000000000000000000123""#, r#"true"#];
        ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
            => Some(::serde_json::from_str::<Block<Transaction>>(EXAMPLE_BLOCK).unwrap())
    );
}
