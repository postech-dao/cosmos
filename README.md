# Cosmos-colony
This repository implements the colony chain interface for Cosmos that can interact with PDAO blockchain.

## Development

### Setup Rust
```sh
rustup default stable
cargo version
# If this is lower than 1.55.0+, update
rustup update stable

rustup target list --installed
rustup target add wasm32-unknown-unknown
```

### Build
```
cargo build --all
```

### Format
Install the nightly version of rustfmt
```
rustup toolchain install nightly
```
To install:
```
rustup component add rustfmt --toolchain nightly
```
To run:
```
cargo +nightly fmt
```

### Lint
Install the nightly version of rustfmt
```
rustup toolchain install nightly
```
To install:
```
rustup component add clippy --toolchain nightly
```
To run:
```
cargo clippy --all --all-targets --release
```

### Test
#### Get tokens from the faucet in malaga testnet
```shellscript
./script/malaga_faucet.sh
```


```
TEST_CONFIG=test_config_example.json cargo test --all
```

### Optimize contracts
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```
M1
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.12.6
```

### Public Juno Nodes

```
Mainnet RPC+LCD:
https://rpc-juno.itastakers.com/
https://lcd-juno.itastakers.com/

Testnet SmartContracts Code Explorer:
https://blueprints.juno.giansalex.dev/#/

testnet explorer:
https://explorer.uni.chaintools.tech/uni
https://testnet.mintscan.io/juno-testnet
https://testnet.juno.explorers.guru/

Testnet RPC+LCD:
- https://rpc.uni.junonetwork.io/
- https://api.uni.junonetwork.io/
- https://rpc.uni.juno.deuslabs.fi/
- https://lcd.uni.juno.deuslabs.fi/
- https://rpc.uni.junomint.com:443/
- https://api.uni.junomint.com:443/
- https://uni-api.blockpane.com/ # RPC and LCD
```

### How to contribute
1. Fork this repository.
2. Clone your forked repository in your local environment.
3. Stack commits locally for a single PR (It should represent one topic).
4. After finishing writing code, run [formatter](#format), [linter](#lint), and [test](#test).
5. Push the commits to the remote forked repository.
6. Go to the forked repository web page and make a PR to this repository.
7. Add posgnu and junha1 as reviewers and ping them in the discord server.
