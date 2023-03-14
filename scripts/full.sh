sh ./scripts/build.sh 
sh ./scripts/store.sh
sh ./scripts/instantiate.sh
sh ./scripts/query_supply.sh
sh ./scripts/transfer.sh

# query supply every 3 seconds for 60 seconds 
# (20 times)
for i in {1..20}
do
  sh ./scripts/query_supply.sh
  echo "Sleeping 3 seconds..."
  sleep 3
done