# On-chain metadata and asset generation for ERC721 NFTs

This project is based on the [Chainfusion](https://internetcomputer.org/chainfusion) starter template.

## Get started:

No matter what setup you pick from below, run `./deploys.sh` from the project root to deploy the project. To understand the steps involved in deploying the project locally, examine the comments in `deploy.sh`. This script will

-   start anvil
-   start dfx
-   deploy the EVM contract
-   generate a number of jobs
-   deploy the chainfusion canister

If you want to check that the `chainfusion_backend` really processed the events, you can either look at the logs output by running `./deploy.sh` – keep an eye open for the `Assets & Metadata successfully generated` message – or you can call the EVM contract to get the `tokenURI`.
To do this, run `cast call --rpc-url=127.0.0.1:8545 0x5fbdb2315678afecb367f032d93f642f64180aa3  "tokenURI(uint256)(string)" <token_id>` where `<token_id>` is the id of the token you want to get the `tokenURI` for. This should return `"http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/<token_id>"` for processed mints and `server returned an error response: error code 3: execution reverted: revert: NOT_MINTED` for unprocessed mints.

If you want to mint more NFTs, simply run `cast send --rpc-url=127.0.0.1:8545 0x5fbdb2315678afecb367f032d93f642f64180aa3  "mintTo(address)" <mint_to_address> --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`.

You can learn more about how to use cast [here](https://book.getfoundry.sh/reference/cast/).

### In the cloud:

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/letmejustputthishere/chainfusion-starter/?quickstart=1)

### Locally:

Make sure you have you have Docker and VS Code installed and running, then click the button below

[![Open locally in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/letmejustputthishere/chainfusion-starter)

### Or do the manual setup:

Make sure that [Node.js](https://nodejs.org/en/) `>= 21`, [foundry](https://github.com/foundry-rs/foundry), [caddy](https://caddyserver.com/docs/install#install) and [dfx](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) `>= 0.18` are installed on your system.

Run the following commands in a new, empty project directory:

```sh
git clone https://github.com/letmejustputthishere/chainfusion-nft-creator.git # Download this starter project
cd chainfusion-nft-creator # Navigate to the project directory
```

## Overview

This project demonstrates how to use the Internet Computer as a coprocessor for EVM smart contracts. The coprocessor listens to events emitted by an EVM smart contract, processes them, and optionally sends the results back. Note that way say EVM smart contracts, as you can not only interact with the Ethereum network, but other networks that are using the Ethereum Virtual Machine (EVM), such as Polygon and Avalanche.

This is an early project based on the [Chainfusion](https://internetcomputer.org/chainfusion) starter template and should be considered as a proof of concept. It is not production-ready and should not be used in production environments. There are quite some TODOs in the code which will be addressed over time. If you have any questions or suggestions, feel free to open an issue, start a [discussion](https://github.com/letmejustputthishere/chainfusion-starter/discussions) or reach out to me on the [DFINITY Developer Forum](https://forum.dfinity.org/u/cryptoschindler/summary) or [X](https://twitter.com/cryptoschindler).

## What is a coprocessor?

The concept of coprocessors originated in computer architecture as a technique to enhance performance. Traditional computers rely on a single central processing unit (CPU) to handle all computations. However, the CPU became overloaded as workloads grew more complex.

Coprocessors were introduced to offload specific tasks from the CPU to specialized hardware. We see the same happening in the EVM ecosystem. EVM smart contracts, and Ethereum in particular, are a very constrained computing environment. Coprocessors and stateful Layer 2 solutions enable to extend the capabilities of the EVM by offloading specific tasks to more powerful environments.

You can read more about coprocessors in the context of Ethereum in the article ["A Brief Into to Coprocessors"](https://crypto.mirror.xyz/BFqUfBNVZrqYau3Vz9WJ-BACw5FT3W30iUX3mPlKxtA). The first paragraph of this section was directly taken from this article.

## Why use ICP as a coprocessor for Ethereum?

Canister smart contracts on ICP can securely read from EVM smart contracts (using [HTTPS Outcalls](https://internetcomputer.org/https-outcalls) or the [EVM RPC](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview) canister) and write to them (using Chain-key Signatures, i.e. [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa)). Hence, there are no additional parties needed to relay messages between the two networks, and no additional work needs to be done on the EVM side to verify the results of the computation as the EVM smart contract just needs to check for the proper sender.

Furthermore, canister smart contracts have many capabilities and properties that can be leveraged to extend the reach of smart contracts:

-   WASM Runtime, which is much more efficient than the EVM, and allows programming in [Rust, JavaScript, and other traditional languages](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/write/overview#choosing-the-programming-language-for-the-backend) (next to [Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko/)).
-   [400 GiB of memory](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/best-practices/storage/) with the cost of storing 1 GiB on-chain for a year only being $5
-   [Long-running computations](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/resource-limits/) that even allow [running AI inference](https://x.com/dominic_w/status/1770884845570326589).
-   [HTTPS Outcalls](https://internetcomputer.org/docs/current/references/https-outcalls-how-it-works) allow canisters to interact with other chains and traditional web services.
-   [Chain-key signatures](https://internetcomputer.org/docs/current/references/t-ecdsa-how-it-works) allow canisters to sign transactions for other chains, including Ethereum and Bitcoin.
-   [Timers](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/periodic-tasks/) allow syncing with EVM events and scheduling other tasks.
-   [Unbiasable randomness](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/randomness/) provided by the threshold BLS signatures straight from the heart of [ICP's Chain-key technology](https://internetcomputer.org/how-it-works/chain-key-technology/).
-   [Serve webcontent](https://internetcomputer.org/how-it-works/smart-contracts-serve-the-web/) directly from canisters via the [HTTP gateway protocol](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/serving-http-request)
-   The [reverse gas model](https://internetcomputer.org/docs/current/developer-docs/gas-cost/#the-reverse-gas-model) frees end users from paying for every transaction they perform
-   ~1-2 second [finality](https://internetcomputer.org/how-it-works/consensus/)
-   [Multi-block transactions](https://internetcomputer.org/capabilities/multi-block-transactions/)

For more context on how ICP can extend Ethereum, check out [this presentation](https://docs.google.com/presentation/d/1P9wycxRsJ6DM_c8TbZG4Xun5URYZbk3WALS4UpSH0iA/edit?usp=sharing) from EthereumZuri 2024.

## Architecture

![image](https://github.com/letmejustputthishere/chainfusion-starter/assets/32162112/7947d2f1-bbaa-4291-b089-2eb05c5d42df)

### EVM Smart contract

The contract `src/foundry/NFT.sol` is a simple implementation of a [ERC721 contract](https://eips.ethereum.org/EIPS/eip-721) that emits an `Transfer` event when the `mintTo` function is called. The `mintTo` function accepts an `address` argument that is used to decide the recipient of the minted NFT.

```solidity
    function mintTo(address recipient) public payable returns (uint256) {
        uint256 newItemId = ++currentTokenId;
        // this emits the following event
        // Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
        _safeMint(recipient, newItemId);
        return newItemId;
    }
```

The contract also has a `tokenURI` function that returns a distinct Uniform Resource Identifier (URI) for a given asset. The `_baseURI` that is passed as a constructor upon contract creation is set to the [URL of the chainfusion canister](https://internetcomputer.org/how-it-works/smart-contracts-serve-the-web/). The `tokenURI` function returns the base URI concatenated with the token ID and throws if `tokenId` is not a valid NFT.

```solidity
    function tokenURI(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        if (ownerOf(tokenId) == address(0)) {
            revert NonExistentTokenURI();
        }
        return
            bytes(baseURI).length > 0
                ? string(abi.encodePacked(baseURI, tokenId.toString()))
                : "";
    }
```

The source code of the contract can be found in `src/foundry/NFT.sol`.

For local deployment of the EVM smart contract and submitting transactions we use [foundry](https://github.com/foundry-rs/foundry). You can take a look at the steps needed to deploy the contract locally in the `deploy.sh` script which runs `script/NFT.s.sol`. Make sure to check both files to understand the deployment process.

### Chainfusion canister

The `chainfusion_backend` canister listens to events emitted by the Ethereum smart contract by [periodically calling](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/periodic-tasks/#timers) the `eth_getLogs` RPC method via the [EVM RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister). When an event is received, the canister can do all kinds of synchronous and asynchronous processing. In this project, the chainfusion canister leverages the [unbiasable randomness](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/randomness/) provided by the ICP to randomly generate metadata and assets for an NFT – fully on-chain. You can learn more about how the EVM RPC canister works and how to integrate with it [here](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview).

The logic for the job that is run on each event can be found in `src/chainfusion_backend/job.rs`. You can find the code for the generators in `src/chainfusion_backend/src/job/generators.rs`.

```rust
pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // Transfer events with the from topic set to the zero address
    // we can safely assume that the event is a mint event.
    let mint_event = MintEvent::from(event);
    // we get secure random bytes from the IC to seed the RNG
    // for every mint event.
    // you can read more [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/randomness/)
    let random_bytes = get_random_bytes().await;
    let mut rng = ChaCha20Rng::from_seed(random_bytes);
    // using th random number generator seeded with the on-chain random bytes
    // we generate our attributes for the NFT
    let attributes = generate_attributes(&mut rng);
    // based on the attributes we generate and store the opensea compliant
    // metadata in the canisters stable memory.
    // canister can currently access 400GB of mutable on-chain storage, you can read more about
    // this feature [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/storage/).
    generate_and_store_metadata(&mint_event, &attributes);
    // last, based on the attributes we generate and store the image in the canisters stable memory.
    generate_and_store_image(&mint_event, &attributes);
    println!("Assets & Metadata successfully generated: http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/{:?}", &mint_event.token_id);
}
```

## Develop

The Chainfusion canister has been structured in a way that all the coprocessing logic lives in `src/chainfusion_backend/src/job.rs` and developers don't need to recreate or touch the code responsible for fetching new events, creating signatures or sending transactions. They can solely focus on writing jobs to run upon receiving a new event from an EVM smart contract.

You can find the full flow for this use-case in the following sequence diagram with Ethereum as an example EVM chain (note that this flow can be applied to any EVM chain):

![image](https://github.com/letmejustputthishere/chainfusion-nft-creator/assets/32162112/d8e8520b-8676-4dd3-aa96-b6bcc790897b)

## Chainfusion starter use-cases

Here you can find a number of examples leveraging the Chainfusion starter logic:

-   On-chain asset and metadata creation for ERC721 NFT contracts (this project)

Build your own use-case on top of the Chainfusion starter and [share it with the community](https://github.com/letmejustputthishere/chainfusion-starter/discussions/10)! Some ideas you could explore:

-   A referral canister that distributes rewards to users based on their interactions with an EVM smart contract
-   A ckNFT canister that mints an NFT on the ICP when an EVM helper smart contract emits an `ReceivedNft`, similar to the [`EthDepositHelper`](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/minter/EthDepositHelper.sol) contract the ckETH minter uses. This could enable users to trade NFTs on the ICP without having to pay gas fees on Ethereum.
