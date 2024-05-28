// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.10;

import "solmate/tokens/ERC721.sol";
import "openzeppelin-contracts/contracts/utils/Strings.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";

error MintPriceNotPaid();
error MaxSupply();
error NonExistentTokenURI();
error WithdrawTransfer();
error RerollPriceNotPaid();
error NotOwnerOfToken();
error NotCoprocessorCanister();

contract NFT is ERC721, Ownable {
    /// @dev This event emits when the metadata of a token is changed.
    /// So that the third-party platforms such as NFT market could
    /// timely update the images and related attributes of the NFT.
    event MetadataUpdate(uint256 _tokenId);

    using Strings for uint256;
    string public baseURI;
    address payable private immutable coprocessor;
    uint256 public currentTokenId;
    uint256 public constant TOTAL_SUPPLY = 10_000;
    uint256 public constant MINT_PRICE = 0.08 ether;
    uint256 public constant REROLL_PRICE = 0.01 ether;

    constructor(
        string memory _name,
        string memory _symbol,
        string memory _baseURI,
        address _coprocessor
    ) ERC721(_name, _symbol) Ownable(msg.sender) {
        baseURI = _baseURI;
        coprocessor = payable(_coprocessor);
    }

    function mintTo(address recipient) public payable returns (uint256) {
        if (msg.value != MINT_PRICE) {
            revert MintPriceNotPaid();
        }
        uint256 newTokenId = currentTokenId + 1;
        if (newTokenId > TOTAL_SUPPLY) {
            revert MaxSupply();
        }
        currentTokenId = newTokenId;
        // this emits the following event
        // Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
        _safeMint(recipient, newTokenId);
        return newTokenId;
    }

    function reroll(uint256 tokenId) public payable {
        // check if the caller is the owner of `tokenId`
        if (msg.sender != ownerOf(tokenId)) {
            revert NotOwnerOfToken();
        }
        if (msg.value != REROLL_PRICE) {
            revert RerollPriceNotPaid();
        }
        // Forward the ETH received to the coprocessor address
        // To pay for the submission of the job result back to the EVM
        // contract.
        coprocessor.transfer(msg.value);
        emit MetadataUpdate(tokenId);
    }

    function rerollFailed(uint256 tokenId) public {
        if(msg.sender != coprocessor) {
            revert NotCoprocessorCanister();
        }
        _burn(tokenId);
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        if (ownerOf(tokenId) == address(0)) {
            revert NonExistentTokenURI();
        }
        return
            bytes(baseURI).length > 0
                ? string(abi.encodePacked(baseURI, tokenId.toString()))
                : "";
    }

    function withdrawPayments(address payable payee) external onlyOwner {
        if (address(this).balance == 0) {
            revert WithdrawTransfer();
        }

        payable(payee).transfer(address(this).balance);
    }

    function _checkOwner() internal view override {
        require(msg.sender == owner(), "Ownable: caller is not the owner");
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override(ERC721) returns (bool) {
        return
            interfaceId == bytes4(0x49064906) ||
            super.supportsInterface(interfaceId);
    }
}
