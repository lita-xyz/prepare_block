// This code is modified from SP1, which is based on the original implementation of Zeth.
//
// Reference: https://github.com/risc0/zeth
//
// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactions, BlockTransactionsKind};
use anyhow::Result;
use async_trait::async_trait;
use reth_valida::primitives::alloy2reth::IntoReth;
use reth_valida::primitives::ValidaRethInput;
use std::collections::HashSet;
use url::Url;

#[async_trait]
pub trait ValidaRethInputInitializer {
    /// Initialize [ValidaRethInput] from [ValidaRethArgs].
    async fn initialize(rpc_url: &str, block_number: u64) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait]
impl ValidaRethInputInitializer for ValidaRethInput {
    async fn initialize(rpc_url: &str, block_number: u64) -> Result<Self> {
        // Initialize the provider.
        let provider =
            ProviderBuilder::new().on_http(Url::parse(&rpc_url).expect("invalid rpc url"));

        // Get the block.
        let parent_block = provider
            .get_block(BlockId::from(block_number - 1), BlockTransactionsKind::Hashes)
            .await?;
        let parent_header = parent_block.unwrap().header;
        let block = provider
            .get_block(BlockId::from(block_number), BlockTransactionsKind::Full)
            .await?
            .unwrap();
        let input = ValidaRethInput {
            timestamp: block.header.timestamp,
        };
        // DONE!

        Ok(input)
    }
}
