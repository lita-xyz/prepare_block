This crate is to be used with [`reth-valida`](https://github.com/lita-xyz/reth-valida). It initializes a requested block and prepares it to be read by `reth-valida`. This version is tested to be compatible with `reth-valida` v0.7.0-alpha.

## Usage

```
cargo run -- --rpc-url "<RPC URL>" --block-number <block number>
```

E.g.,

```
cargo run -- --rpc-url "https://eth.llamarpc.com" --block-number 17106222
```
