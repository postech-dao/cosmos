# cosmos

## Reference
[Juno](https://docs.junonetwork.io/juno/readme)
[cosmwasm](https://docs.cosmwasm.com/docs/1.0/)
[cosmos sdk](https://docs.cosmos.network/)

## Run
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
```
cargo test -all
```