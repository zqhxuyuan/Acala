#!/bin/bash

set -ex

args=(
  --locked \
  --features with-karura-runtime \
  -- \
  -lruntime=debug \
  -levm=debug \
  -lrevenue=debug \
  --chain=./launch/output/karura-dev-2000.json \
  --tmp \
  #--base-path=./data \
  --ws-external=true \
  --ws-port=19944 \
  --rpc-external=true \
  --rpc-port=19933 \
  --rpc-cors=all \
  --name=parachain-2000-10 \
  #--collator
  --validator
  --parachain-id=2000 \
  --rpc-methods=unsafe \
  --force-authoring \
  --wasm-execution=compiled \
  --execution=native \
  --eve \
  --bootnodes=/ip4/192.168.50.62/tcp/30336/p2p/12D3KooWHRkydeVjms3xGqDnDNWsZLvqjFs6PjYPGJCZk3Tz92Ug \
  --listen-addr=/ip4/0.0.0.0/tcp/31333 \
  -- \
  --chain=./launch/output/rococo-local.json \
  --wasm-execution=compiled \
  --execution=native \
  --bootnodes=/ip4/192.168.50.62/tcp/30333/p2p/12D3KooWJR7SQ747mVBJXt6hJamuYRKvP7Ye2TC9FxQKEU3rmzQZ \
  --no-beefy
)

cd ..
cargo run "${args[@]}"
