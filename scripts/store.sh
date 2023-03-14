VIRUS_PATH="./artifacts/ics20_devils_snare.wasm"
KEY="game-of-nfts-sh"
junod tx wasm store $VIRUS_PATH \
  --from $KEY \
  --gas-prices 0.025ujuno \
  --gas "auto" \
  --gas-adjustment 2 \
  --yes \
  --output json \
  --node https://juno-rpc.polkachu.com:443 \
  --chain-id juno-1 \
  > ics20-devils-snare-store-juno.log