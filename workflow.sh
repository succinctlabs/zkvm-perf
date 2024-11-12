#! /bin/bash

INSTANCES=(
    "aws-g6-xlarge"
    "aws-g6-2xlarge"
    "aws-g6-16xlarge"
)
PROVERS=(
    "sp1"
    "risc0"
)
SHARD_SIZES=(
    "21"
)
PROGRAMS="loop10k,loop100k,loop1m,loop3m,loop10m,loop30m,loop100m,fibonacci,tendermint,reth1,reth2,sha256-1mb,sha256-3mb,sha256-10mb,keccak256-1mb,keccak256-3mb,keccak256-10mb"

# Get the current git branch.
GIT_REF=$(git rev-parse --abbrev-ref HEAD)

# Create a JSON object with the specified fields.
WORKLOADS=$(jq -n \
    --arg instances "$(printf '%s\n' "${INSTANCES[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg provers "$(printf '%s\n' "${PROVERS[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg shard_sizes "$(printf '%s\n' "${SHARD_SIZES[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg programs "$PROGRAMS" \
    '{instances: $instances, provers: $provers, shard_sizes: $shard_sizes, programs: $programs}')

# Run the workflow with the list of workloads.
echo $WORKLOADS | gh workflow run suite.yml --ref $GIT_REF --json