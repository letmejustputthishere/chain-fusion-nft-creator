// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/foundry/NFT.sol";

contract MyScript is Script {
    function run(address chain_fusion_canister_address) external {
        // the private key of the deployer is the first private key printed by running anvil
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        // we use that key to broadcast all following transactions
        vm.startBroadcast(deployerPrivateKey);

        // this creates the contract. it will have the same address every time if we use a 
        // new instance of anvil for every deployment.
        // the baseURI is the prefix for the tokenURI. in this case it point to a canister deployed locally.
        // we can infer the canister id here because we specify it in `dfx.json`. usually one would
        // first need to create the canister, deploy the NFT contract passing the canister url as baseURI
        // and then deploying the canister passing the NFT contract address as a deploy argument.
        NFT nft = new NFT("Chainfusion", "CF", "http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/", chain_fusion_canister_address);

        // we can call the mint function to mint a token
        // the address we mint to belongs to the deployerPrivateKey
        // again, the transaction is signed with the deployerPrivateKey
        for (uint256 index = 0; index < 10; index++) {
            nft.mintTo{value: 0.08 ether}(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
        }

        vm.stopBroadcast();
    }
}
