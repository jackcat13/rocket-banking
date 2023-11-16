# Rust + Rocket sample project

Code coverage : [![codecov](https://codecov.io/gh/jackcat13/rocket-banking/branch/main/graph/badge.svg?token=f7d9ac1a-2e90-4f85-ad5d-badfd30ef4f0)](https://codecov.io/gh/jackcat13/rocket-banking)

## Purpose

The purpose of this project is to showcase a functional example of a REST micro-service. It aims to work as an aggregator of multiple BANK accounts balances in order to make fast the visualisation of several BANK accounts.

The following diagram shows how the service behaves :

```mermaid
flowchart LR

A[Third party client] --> |GET /banks| B(Rocket Banking microservice)
B <--> C{For each BANK credential}
C --> |Get Bank account 1 informations| D[Bank 1 API]
C --> |Get Bank account 2 informations| E[Bank 2 API]
D --> |Account 1 details| C
E --> |Account 2 details| C
B --> |Aggregated accounts details| A
```

:notebook: Each BANK credential is provided in the HTTP request header `credentials`. (TODO : wrap credentials in an encrypted token. Not done yet because it is an example project not meant for production).

## Run the microservice

### Using docker compose

1. [Install docker](https://docs.docker.com/desktop/)
2. Clone this repository
3. go to `compose` folder
4. Run `docker compose up` command

### Build from sources

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository
3. Run `cargo run` command

:warning: If any third party is required (like a DB or another dependant service), these won't be deployed by default.

## Test the microservice

Once the microservice is started, you can test it with the Mocked bank account using the following command :

```sh
curl http://localhost:8000/banks -H 'credentials: { "basic" : [{ "identifier": "1", "password": "1", "bank": "Mock" }] }'
```
