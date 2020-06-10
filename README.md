# 

# Dipole Oracle

![](./img/dipole.jpeg)

<!-- TOC -->

- [1. Introduction](#1-introduction)
- [2. Overview](#2-overview)
- [3. Build](#3-build)
- [4. Run](#4-run)
- [5. Examples](#5-Examples)

<!-- /TOC -->

# 1. Introduction

This project is initiated and facilitated by [Dipole Tech](https://dipole.tech/). Dipole Tech is Distributed Energy Resource aggregator, providing services for the massive amount of distributed energy assets that will emerge in the future and facilitating the decarbonization of the energy system. Dipole Tech develops an independent Substrate-based blockchain which records all stages within the industry and enables interoperability from DER assets. 

DipoleOracle connects Dipole chain and offline electrical hardware, ensure the safety and accuracy of  offline power usage and transaction data, make them applicable for blockchain use.  Which can enrich  the ecosystem of Substrate and Polkadot, and bring  the  revolution of clean energy ecology in whole society.

Dipole Oracle aims to build a reliable and efficient platform to connect the offline power usage data and power transaction data to blockchain service. Dipole Oracle's mission is to accelerate the clean energy ecology revolution, and enrich the ecology of [Substrate](https://substrate.dev/) and [Polkadot](https://polkadot.network/).

# 2. Overview

DipoleOracle  includes four key components: Operator, GoodsOracle, PayOracle and Collector. The whole system provides the feeding and collecting of energy generation/consumption and transaction data.

![](./img/dipoleoracle.png)



# 3. Build



Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

# 4. Run

Purge any existing developer chain state:

```bash
./target/release/dipole-oracle purge-chain --dev
```

Start a development chain with:

```bash
./target/release/dipole-oracle --dev
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/dipole-oracle --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/dipole-oracle --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/dipole-oracle purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

# 5. Examples

An example shows how to use Dipole Oracle with nodejs are [here](./examples/nodejs).

