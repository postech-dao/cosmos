#!/bin/bash

# Configure
source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/defaults.env)

# Use the below malaga testnet account
:'
Sender publickey {"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Aggx3Gp4SJOHzZK4WDen/j5EXutf78JB87DQA5/7Z59y"}
Sender account id wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf
Mnemonic "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry"
'

# Send request for tokens. You should see ok message after executing this command
curl -X POST --header "Content-Type: application/json" --data '{"denom":"umlg","address":"wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf"}' https://faucet.malaga-420.cosmwasm.com/credit

# Check that your faucet request has been successful
wasmd query bank balances wasm1quzyfdgzw42aelcdkrw2v8vnfdxsk9jkl7a4qf --node https://rpc.malaga-420.cosmwasm.com:443

# You can see something like this
:'
balances:
- amount: "100000000"
  denom: umlg
pagination:
  next_key: null
  total: "0"

'