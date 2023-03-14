KEY="game-of-nfts-sh"
CONTRACT_ADDRESS=$(\
  cat ./ics20-devils-snare-instantiate-juno.log | \
  jq -r '.logs[].events | 
          map(select(.type == "instantiate")) | 
          .[0].attributes | 
          map(select(.key == "_contract_address")) | 
          .[0].value'\
)
echo "Code ID: $CODE_ID"

junod tx wasm execute $CONTRACT_ADDRESS '{ 
    "transfer": {
      "amount": "1",
      "recipient": "juno105ak6rcqd7sjwr8308hnannh267wvm9u2jgczepk84p9f537d5vqujsp99"
    }
  }' \
  --from $KEY \
  --gas-prices 0.025ujuno \
  --gas "auto" \
  --gas-adjustment 2 \
  --yes \
  --output json \
  --node https://juno-rpc.polkachu.com:443 \
  --chain-id juno-1 \
  | jq > ics20-devils-snare-transfer-juno.json