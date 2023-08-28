# ZK-Bench

zk benchmarks for popular zk frameworks

## Why?

We created zk-bench to help support and grow the zk community. The zk space is growing incredibly fast (which is great 🔥), but that makes it harder for builders to understand the tooling and trade-offs of different frameworks. 

Our goal is not to discover or evaluate "the best zk framework", there probably isn't one. Instead, we want to give you the data, so you can find the best framework for your use case. 

## Benchmarks are not fair

Benchmarks are really hard. We've done our best to provide accurate and fair benchmarks, and we'll happily accept contributions if you think this can be improved. We also strive to caveat and explain the results as best we can, so developers/builders/researchers have a full understanding of trade-offs.


## Install

### Risc Zero

In order to run Risc Zero, you need to install the SDK/toolchain.

Install `risczero` as a cargo sub-command:

```bash
cargo install cargo-risczero
```

Install the risczero toolchain:

```
cargo risczero install
```
