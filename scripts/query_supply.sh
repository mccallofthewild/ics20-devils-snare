CONTRACT_ADDRESS=$(\
  cat ./ics20-devils-snare-instantiate-juno.log | \
  jq -r '.logs[].events | 
          map(select(.type == "instantiate")) | 
          .[0].attributes | 
          map(select(.key == "_contract_address")) | 
          .[0].value'\
)
junod q wasm cs smart \
  $CONTRACT_ADDRESS '{
    "token_info": {}
  }' \
  --node https://juno-rpc.polkachu.com:443 \
  --chain-id juno-1 \
  --output json \
  | jq > ics20-devils-snare-supply-juno.json


junod q wasm cs smart \
  $CONTRACT_ADDRESS '{
    "balance": {
      "address": "juno1v4887y83d6g28puzvt8cl0f3cdhd3y6y9mpysnsp3k8krdm7l6jqgm0rkn"
    }
  }' \
  --node https://juno-rpc.polkachu.com:443 \
  --chain-id juno-1 \
  --output json \
  | jq

cat ics20-devils-snare-supply-juno.json | jq;