// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

contract IPFSCIDStorage {
	mapping(address => string) CIDMap;

	function store(string memory ipfsCID) public {
		CIDMap[msg.sender] = ipfsCID;
	}

	function get() public view returns (string memory) {
		string memory storedCID = CIDMap[msg.sender];
		require(bytes(storedCID).length > 0, "No CID stored for this user");
		return storedCID;
	}
}
