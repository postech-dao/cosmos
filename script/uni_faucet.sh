#!/bin/bash

# reference: https://docs.junonetwork.io/validators/joining-the-testnets

# Configure
source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/uni-3/defaults.env)

# make wallet
#junod keys add test_uni3
#keyring passphrase: testuni3

# Use the below malaga testnet account
:'
name: test_uni3
type: local
address: juno1x950tdufany5aluq7y93e2skprwj7t6quwv8z9
pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Al0MH+ebce5CUODiyNYEQkvf0e3LuW4jbqD2s/kKBKFF"}'
mnemonic: "embody accuse hour soul cream trick cabbage door where matrix shed hand level figure excuse input shove screen amateur forward floor crack wash mango"
'

# Send request for tokens. You should see ok message after executing this command
curl -X POST --header "Content-Type: application/json" --data '{"denom":"ujunox","address":"juno1x950tdufany5aluq7y93e2skprwj7t6quwv8z9"}' https://rpc.uni.juno.deuslabs.fi/credit

# Check that your faucet request has been successful
junod query bank balances juno1x950tdufany5aluq7y93e2skprwj7t6quwv8z9 --node https://rpc.uni.juno.deuslabs.fi:443

# You can see something like this
:'
balances:
- amount: "10000000"
  denom: ujunox
pagination:
  next_key: null
  total: "0"
'