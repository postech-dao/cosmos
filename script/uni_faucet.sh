#!/bin/bash

# reference: https://docs.junonetwork.io/validators/joining-the-testnets

# Configure
source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/uni-3/defaults.env)

# make wallet
#junod keys add test_uni3
#keyring passphrase: testuni3

# Use the below uni-3 testnet account
:'
name: test_uni3
type: local
address: juno18sfvnktcdvufrngsy7nhdjc6z84lz32xqjky4j
pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AlrEj+tQ8rQXG0IXhwWGN6QaA01HwmmvjlKI1M89Y3LD"}'
mnemonic: "youth amused accident boring boss sniff solid inmate small body slow surround survey have rough pill risk ankle extra useful slush junk rally slogan"
'

# Send request for tokens. You should see ok message after executing this command
curl -X POST --header "Content-Type: application/json" --data '{"denom":"ujunox","address":"juno18sfvnktcdvufrngsy7nhdjc6z84lz32xqjky4j"}' https://faucet.uni.juno.deuslabs.fi/credit

# Check that your faucet request has been successful
junod query bank balances juno18sfvnktcdvufrngsy7nhdjc6z84lz32xqjky4j --node https://rpc.uni.juno.deuslabs.fi:443

# You can see something like this
:'
balances:
- amount: "10000000"
  denom: ujunox
pagination:
  next_key: null
  total: "0"
'