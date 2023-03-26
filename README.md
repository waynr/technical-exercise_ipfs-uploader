# ChainSafe Systems Technical Assessment

# Usage

## Development Environment

**Please note that this has only been tested on Arch Linux and any compatibility
with mac or windows is incidental.**

The development environment is required to do anything with either the CLI or
the e2e tests. To use it, you will have to docker and [install docker
compose](https://docs.docker.com/compose/install/).


```
docker compose up
```

This should start a kubo (IPFS Node) instance and an Anvil (Ethereum Node meant
for local dev) in containers using your local docker daemon. The following
ports will be exposed on localhost:

* 4001/udp & 4001/tcp
  * "swarm" listener
* 5001
  * web ui & api server
* 8080
  * read-only gateway?

# TODO:

* [x] review documentation & plan implementation
  * learn basics of IPFS
  * learn basics of smart contracts and ethereum
  * outline project
  * create cargo project
* [ ] set up development environment
  * install docker compose
  * create compose.yml that runs;
    * an ipfs node 
      * https://docs.ipfs.tech/how-to/command-line-quick-start/
      * https://docs.ipfs.tech/install/run-ipfs-inside-docker/
        * ipfs/kubo
    * ~~ganache: https://github.com/trufflesuite/ganache~~
    * Anvil 
      * ghcr.io/foundry-rs/foundry:latest
* [ ] write simple smart contract with single function that sets the value on a
    variable whose type is capable of representing CIDs
* [ ] write CLI to:
  * [ ] upload file to ipfs network 
    * to kubo running in docker container
    * using https://github.com/ferristseng/rust-ipfs-api
  * [ ] deploy contract:
    * compile solidity contract & generate rust ABI
    * see https://github.com/gakonst/ethers-rs/blob/master/examples/contracts/examples/deploy_from_abi_and_bytecode.rs
  * [ ] call smart contract function to set CID using CID returned when
      uploading file to IPFS
* [ ] document how to use cli
* [ ] write an e2e test

# Notes

* Assignment suggests using [ganache](https://github.com/trufflesuite/ganache)
  as a local env EVM network node but examples in
  [ethers-rs](https://github.com/gakonst/ethers-rs) (also recommended by
  assignment PDF) assume the use of Anvil.
  May be confusing for candidates.

