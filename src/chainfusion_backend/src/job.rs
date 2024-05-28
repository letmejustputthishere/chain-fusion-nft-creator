mod distribution;
mod generators;
mod submit_result;
use std::fmt;

use ethers_core::types::{Address, U256};
use ic_cdk::{api::management_canister::main::raw_rand, println};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use submit_result::submit_result;

use crate::{
    evm_rpc::LogEntry,
    job::generators::{generate_and_store_image, generate_and_store_metadata, generate_attributes},
    state::{mutate_state, LogSource},
    storage::remove_asset,
};
use std::str::FromStr;

pub async fn job(event_source: LogSource, log_entry: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // Transfer events with the from topic set to the zero address
    // we can safely assume that the event is a mint event.
    let event = log_entry.into_event();
    let random_bytes = get_random_bytes().await;
    match event {
        Event::MintEvent(mint_event) => {
            let mut rng = ChaCha20Rng::from_seed(random_bytes);
            // using th random number generator seeded with the on-chain random bytes
            // we generate our attributes for the NFT
            let attributes = generate_attributes(&mut rng);
            // based on the attributes we generate and store the opensea compliant
            // metadata in the canisters stable memory.
            // canister can currently access 400GB of mutable on-chain storage, you can read more about
            // this feature [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/storage/).
            generate_and_store_metadata(mint_event.token_id, &attributes);
            // last, based on the attributes we generate and store the image in the canisters stable memory.
            generate_and_store_image(mint_event.token_id, &attributes);
            println!("Assets & Metadata successfully generated: http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/{:?}", &mint_event.token_id);
        }
        Event::MetadataUpdateEvent(metadata_update) => {
            println!(
                "Found reroll event for token_id: {:?}",
                metadata_update.token_id
            );
            if reroll_failed(random_bytes[0]) {
                remove_asset(&format!("/{:?}", metadata_update.token_id));
                remove_asset(&format!("/{:?}.svg", metadata_update.token_id));
                remove_asset(&format!("/{:?}.png", metadata_update.token_id));
                submit_result(metadata_update.token_id).await;
            } else {
                let mut rng = ChaCha20Rng::from_seed(random_bytes);
                // using th random number generator seeded with the on-chain random bytes
                // we generate our attributes for the NFT
                let attributes = generate_attributes(&mut rng);
                // based on the attributes we generate and store the opensea compliant
                // metadata in the canisters stable memory.
                // canister can currently access 400GB of mutable on-chain storage, you can read more about
                // this feature [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/storage/).
                generate_and_store_metadata(metadata_update.token_id, &attributes);
                // last, based on the attributes we generate and store the image in the canisters stable memory.
                generate_and_store_image(metadata_update.token_id, &attributes);
                println!("Assets & Metadata successfully rerolled: http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/{:?}", metadata_update.token_id);
            }
        }
    }
}

fn reroll_failed(byte: u8) -> bool {
    byte > 51 // 20% of 255 is approximately 51
}

// This function asynchronously retrieves a random byte array of length 32.
async fn get_random_bytes() -> [u8; 32] {
    // Call the `raw_rand` function and await its result.
    let (raw_rand,) = raw_rand().await.expect("calls to raw_rand should not fail");

    // Convert the obtained byte vector into a fixed-size array of length 32.
    raw_rand
        .try_into()
        .expect("raw_rand should contain 32 bytes")
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MintEvent {
    pub from_address: Address,
    pub to_address: Address,
    pub token_id: U256,
}

pub struct MetadataUpdateEvent {
    pub token_id: U256,
}

pub enum Event {
    MintEvent(MintEvent),
    MetadataUpdateEvent(MetadataUpdateEvent),
}

impl fmt::Debug for MintEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MintEvent")
            .field("from_address", &self.from_address)
            .field("to_address", &self.to_address)
            .field("token_id", &self.token_id)
            .finish()
    }
}

impl LogEntry {
    fn into_event(self) -> Event {
        if self.topics[0] == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef" {
            // we expect exactly 4 topics from the transfer event.
            // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
            let from_address =
                ethers_core::types::Address::from_str(&self.topics[1][self.topics[1].len() - 40..])
                    .expect("the address contained in the first topic should be valid");
            let to_address =
                ethers_core::types::Address::from_str(&self.topics[2][self.topics[1].len() - 40..])
                    .expect("the address contained in the second topic should be valid");
            let token_id =
                U256::from_str_radix(&self.topics[3], 16).expect("the token id should be valid");

            Event::MintEvent(MintEvent {
                from_address,
                to_address,
                token_id,
            })
        } else {
            let token_id =
                U256::from_str_radix(&self.data, 16).expect("the token id should be valid");

            Event::MetadataUpdateEvent(MetadataUpdateEvent { token_id })
        }
    }
}
