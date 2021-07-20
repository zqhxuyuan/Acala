#!/bin/bash

#1.collatorSelection: setCandidacyBond(bond)
#2.collatorSelection: setDesiredCandidates(max)
#3.balances: setBalance(who, new_free, new_reserved)
#4.RPC author_rotateKeys
#curl --location --request POST 'http://localhost:19933' \
#     --header 'Content-Type: application/json' \
#     --data-raw '{
#    "id": 1,
#    "jsonrpc": "2.0",
#    "method": "author_rotateKeys",
#    "params": []
#}'
#5.session setKeys(keys, proof)
#6.collatorSelection: registerAsCandidate()

set -ex

docker run \
  -p 19944:9944 \
  -p 19933:9933 \
  -e RUST_LOG=sc_basic_authorship=trace,cumulus-consensus=trace,cumulus-collator=trace,collator_protocol=trace,collation_generation=trace,aura=debug \
  --rm \
  output_parachain-2000-0 \
  --base-path=/acala/data \
  --chain=/app/karura-dev-2000.json \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --name=parachain-2000-4 \
  --collator \
  --parachain-id=2000 \
  --rpc-methods=unsafe \
  --force-authoring \
  --wasm-execution=compiled \
  --execution=native \
  --eve \
  --bootnodes=/ip4/192.168.50.62/tcp/30336/p2p/12D3KooWBWfyYBsYjjJU8akhvEYmP8rPy7vZUJRw8i7g9QUbvPbN \
  --listen-addr=/ip4/0.0.0.0/tcp/30333 \
  -- \
  --chain=/app/rococo-local.json \
  --wasm-execution=compiled \
  --execution=native \
  --bootnodes=/ip4/192.168.50.62/tcp/30333/p2p/12D3KooWBBW4VWwobUT5nNbvAaJ9JsopXHVA3gXEvdJvcJ1sBsmP \
  --no-beefy
