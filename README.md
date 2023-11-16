# Rust + Rocket sample project

## Purpose

The purpose of this project is to showcase a functional example of a REST micro-service. It aims to work as an aggregator of multiple bank accounts balances in order to make fast the visualisation of several bank accounts.

The following diagram shows how the service behaves :

```mermaid
flowchart LR

A[Third party client] --> |GET /banks| B(Rocket Banking microservice)
B <--> C{For each bank credential}
C --> |Get Bank account 1 informations| D[Bank 1 API]
C --> |Get Bank account 2 informations| E[Bank 2 API]
D --> |Account 1 details| C
E --> |Account 2 details| C
B --> |Aggregated accounts details| A
```

:notebook: Each bank credential is provided in the HTTP request header `credentials`. (TODO : wrap credentials in an encrypted token. Not done yet because it is an example project not meant for production).

## Getting started

TODO

## Tests

This project is tested and metrics are available at following location :