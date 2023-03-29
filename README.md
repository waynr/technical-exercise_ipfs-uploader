# Interview Technical Assessment

# Usage

## Requirements

The following software is assumed to be installed:

* [Docker](https://docs.docker.com/get-docker/)
* [docker compose](https://docs.docker.com/compose/install/).
* [just](https://just.systems/man/en/chapter_2.html)

## Development Environment

**Please note that this has only been tested on Arch Linux**

The development environment is required to do anything with either the CLI or
the e2e tests. To use it, you will have to docker and [install docker
compose](https://docs.docker.com/compose/install/).

In a terminal separate from your editor terminal (assuming you're lame like me
and use vim or emacs):

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

## Building

### Smart Contract

```
just build-contract
```

This takes the smart contract, `./contract.sol`, and produces both a combined
ABI/binary in the `./outputs/` directory that is passed to the
`ethers::contract::abigen` macro for smart contract deployment and interaction
purposes. These will be uploaded to the local Anvil node by the cli.


### CLI

```
cargo build
```

## Running

### Deploy Smart Contract

```
./target/debug/ipfs-uploader --key <key> install-smart-contract
```

* `<key>` can be any of the keys shown in the the `ganache` container stdout
    in the terminal where you ran `docker compose up`.


### Upload File to IPFS & Set CID in Smart Contract

```
./target/debug/ipfs-uploader --key <key> upload --file <file>
```

* `<key>` can be any of the keys shown in the the `ganache` container stdout
    in the terminal where you ran `docker compose up`.
* `<file>` can be any arbitrary file, although there may be some practical file
    size limitations. Also, don't upload files containing personally
    identifying or sensitive information.

# TODO:

* [x] review documentation & plan implementation
  * learn basics of IPFS
  * learn basics of smart contracts and ethereum
  * outline project
  * create cargo project
* [x] set up development environment
  * install docker compose
  * create compose.yml that runs;
    * an ipfs node 
      * https://docs.ipfs.tech/how-to/command-line-quick-start/
      * https://docs.ipfs.tech/install/run-ipfs-inside-docker/
        * ipfs/kubo
    * ~~ganache: https://github.com/trufflesuite/ganache~~
    * Anvil 
      * ghcr.io/foundry-rs/foundry:latest
* [x] write simple smart contract with single function that sets the value on a
    variable whose type is capable of representing CIDs
* [x] write CLI to:
  * [x] upload file to ipfs network 
    * to kubo running in docker container
    * using https://github.com/ferristseng/rust-ipfs-api
  * [x] deploy contract:
    * compile solidity contract & generate rust ABI
    * see https://github.com/gakonst/ethers-rs/blob/master/examples/contracts/examples/deploy_from_abi_and_bytecode.rs
  * [x] call smart contract function to set CID using CID returned when
      uploading file to IPFS
* [x] document how to use cli


## missing functionality

I spent too much time dealing with broken and missing documentation to get this
to the state of completion that I'd prefer:

* [ ] write tests
  * [ ] write e2e tests using docker compose environment
  * [ ] look into best practices for stubbing/mocking network components
  * [ ] write solidity tests for contract
* [ ] implement basic GHA PR workflow
* [ ] add required command line flag to specify chain id (rather than hard coding)
* [ ] add optional command line flags to specify alternative IPFS and ethereum
  endpoints.

# Notes

* Assignment suggests using [ganache](https://github.com/trufflesuite/ganache)
  as a local env EVM network node but examples in
  [ethers-rs](https://github.com/gakonst/ethers-rs) (also recommended by
  assignment PDF) assume the use of Anvil. May be confusing for candidates.

* I did try using `anvil` initially, but was getting random connection closed
  errors on the client side with no corresponding logs on the server side.
  * In retrospect, this was likely a network protocol issue -- I may have been
    erroneously attempting to connect to a tls port with a plaintext http
    client. I've more than exhausted the 4 hours alotted for this exercise and
    the ganache docker image seems to work well enough.

* The acceptable format for the json file taken by `ethers::contract::abigen`
  isn't compatible with the output of `solc --combined-json
  srcmap,abi,bin,opcodes`, so I had to rearrange it using a `jq` filter. (see
  the `compile-contract` command in `./justfile`.

* At the time of implementation, `ethers` docs were borked so the `2.0.1`
  release isn't available on docs.rs. I worked around this by building the repo
  docs locally, but for some reason that I didn't want to spend time debugging
  search in the local version of cargo docs is borked.

* Annoyingly, both `ethers` and `ipfs-api` have error types from other
  libraries in their function signatures which required importing `url` and
  `http` as dependencies. Maybe I could have found them re-exported somewhere
  in those code bases, but I was already running low on time.
