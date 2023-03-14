VIRUS_PATH="./artifacts/ics20_devils_snare.wasm"
KEY="game-of-nfts-sh"
CODE_ID=$(\
  cat ./ics20-devils-snare-store-juno.log | \
  jq -r '.logs[].events | 
          map(select(.type == "store_code")) | 
          .[0].attributes | 
          map(select(.key == "code_id")) | 
          .[0].value'\
)
echo "Code ID: $CODE_ID"

junod tx wasm instantiate $CODE_ID '{ 
    "channel_id":"channel-47",
    "bridge_contract":"juno1v4887y83d6g28puzvt8cl0f3cdhd3y6y9mpysnsp3k8krdm7l6jqgm0rkn",
    "receiver":"osmo1ztxwfpggyc5eqhev548z8wkwj8hx5k4hm59ptq"
  }' \
  --label "Devil's Snare Whitehat Demo" \
  --admin "juno1ztxwfpggyc5eqhev548z8wkwj8hx5k4h9a426w" \
  --from $KEY \
  --gas-prices 0.025ujuno \
  --gas "auto" \
  --gas-adjustment 2 \
  --yes \
  --output json \
  --node https://juno-rpc.polkachu.com:443 \
  --chain-id juno-1 \
  > ics20-devils-snare-instantiate-juno.log