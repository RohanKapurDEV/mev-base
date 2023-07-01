# mev-base

This repo is a cloneable starter template for Jito focused MEV bots on Solana. It is only opinionated on dependency structure, and leaves bot implementation up to the operator. The point of it is to do away with the boilerplate of setting up searcher and geyser protobufs for every new bot, and also to pin versions for the shared dependencies between the `searcher-client`, `geyser-client`, and `geyser-protos` crates.

All included crates are from the [Jito Labs](https://github.com/jito-labs) and [Jito Foundation](https://github.com/jito-foundation) GitHub orgs. All code belongs to them.

## Setting up

```base
git clone https://github.com/RohanKapurDEV/mev-base
cd mev-base/
git submodule update --init --recursive
cargo build
```

At thsi point, you should be good to use the searcher and geyser protobufs and client impls in your bot. You can add your bot to the codebase by running `cargo new --lib <BOT_NAME>`, and adding the bot name to the Cargo workspace file (root level `Cargo.toml`).
