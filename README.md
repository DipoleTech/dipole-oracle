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

DipoleOracle connects Dipole chain and offchain electrical hardware, ensure the safety and accuracy of  offchain power usage and transaction data, make them applicable for blockchain use.  Which can enrich  the ecosystem of Substrate and Polkadot, and bring  the  revolution of clean energy ecology in whole society.

Dipole Oracle aims to build a reliable and efficient platform to connect the offchain power usage data and power transaction data to blockchain service. Dipole Oracle's mission is to accelerate the clean energy ecology revolution, and enrich the ecology of [Substrate](https://substrate.dev/) and [Polkadot](https://polkadot.network/). Onchain authorization, offchain upload and exchange, fine-grained permission control for data upload and sharing，based on the unforgeable characteristics of the blockchain, the data exchange records are put on the chain to ensure that the data sovereignty can be traced in the later stage, and the dispute on the correctness of the shared data can be solved.

# 2. Overview.



DipoleOracle  includes four key components: Operator, GoodsOracle, PayOracle and Collector. The whole system provides the feeding and collecting of energy generation/consumption and transaction data.

<img src="./img/dipoleoracle.png" style="zoom:150%;" />

- Operator

In the energy industry, electrical hardwares are extremely miscellaneous. The Operator module defines several common used electrical hardwares like electricity meter and charging point. The Operator module allows each registered electrical hardware to upload data online.

- GoodsOracle


In the energy industry, collecting real-time data from hardware such as electricity meter and charging piles has always been a technical challenge. DipoleGoodsOracle can be applicable to various electricity meters and protocols, which achieve a solid progress in this area.

- PayOracle


In the energy industry, periodical power transaction cause high needs of splitting bills for participants, third-party payments or stablecoins. PayOracle is able to complete all steps automatically. In area which don't have completed payment system like South Asia and Africa, PayOracle can enormously reduce the cost of payment. 

- Collector

In the energy industry, collecting energy generation/consumption and transaction data costs a lot of manpower and material resources. With the Collector module, users can easily get data they really need, which can help them customize their business scenarios.

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

An Auto Electric Meter Data Collector shows how to use Dipole Oracle with nodejs are [here](./examples/nodejs). 

[electric-meter-hardware-connector](./examples/electric-meter-hardware-connector) is a simulation of an electric meter, which sends raw electric meter data to dipole oracle chain. It shows how to register operator and send raw electric meter data. Under the real environment，developers can  refer to it to connect electric meter hardware and dipole oracle.

[electric-meter-data-shower](./examples/electric-meter-data-shower) shows user's auto electric meter data. Under the real environment，developers can refer to it to develop higher function based on the basic electric meter data.

![](./img/example.png)

#### 5.1 Run

1)run dipole-oralce based on Section3&4

2)run  [electric-meter-hardware-connector](./examples/electric-meter-hardware-connector/README.md)

3)run [electric-meter-data-shower](./examples/electric-meter-data-shower/README.md) 

#### 5.2 Specification

After 5.1, An Auto Electric Meter Data will be shown in http://localhost:8000/ as follows. And It will be updated automatically as the service of [electric-meter-hardware-connector](./examples/electric-meter-hardware-connector) . 

<img src="./img/electricdata.png" style="zoom:50%;" />

It shows electricity comsumption data of user 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, which includes StateGrid and CleanEnergy electricity comsumption data.  Electricity comsumption data in three stage of Peak, Flat, and Valley are shown. 

#### 5.3 Benefits

A short comparision with  electric meter data collection using DipoleOralce before and after are as follows.

|                       | before             | After                                                        |
| --------------------- | ------------------ | ------------------------------------------------------------ |
| Electric Meter Device | hard understanding | unique DID in Blockchain Sys， easy understanding &Identifiable & traceable |
| Uploading Data        | mostly human work  | auto uploading with signature                                |
| Supervision           | hard               | easy  traceable                                              |
| Authority Control     | mostly human work  | easy realization with DID & Operator module                  |

