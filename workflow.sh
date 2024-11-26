#! /bin/bash

INSTANCES=(
    "aws-g6-2xlarge"
    # "aws-g6-16xlarge"
)
PROVERS=(
    "sp1"
    # "risc0"
)
SHARD_SIZES=(
    "21"
)
PROGRAMS=(
    "loop10k"
    "loop100k"
    # "loop1m"
    # "loop3m"
    # "loop10m"
    # "loop30m"
    # "loop100m"
    # "loop300m"
    "fibonacci20k"
    "fibonacci200k"
    # "fibonacci2m"
    # "fibonacci4m"
    # "fibonacci20m"
    # "fibonacci40m"
    # "fibonacci200m"
    # "fibonacci400m"
    # "sha256100kb"
    # "sha256300kb"
    # "sha2561mb"
    # "sha2563mb"
    "keccak256100kb"
    "keccak256300kb"
    # "keccak2561mb"
    # "keccak2563mb"
    # "keccak25610mb"
    # "ssz-withdrawals"
    # "tendermint"
    # "reth1"
    # "reth2"
    "ecdsa-verify"
    # "eddsa-verify"
)

# Get the current git branch.
GIT_REF=$(git rev-parse --abbrev-ref HEAD)

# Create a JSON object with the specified fields.
WORKLOADS=$(jq -n \
    --arg instances "$(printf '%s\n' "${INSTANCES[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg provers "$(printf '%s\n' "${PROVERS[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg shard_sizes "$(printf '%s\n' "${SHARD_SIZES[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    --arg programs "$(printf '%s\n' "${PROGRAMS[@]}" | jq -R . | jq -s 'map(select(length > 0))')" \
    '{instances: $instances, provers: $provers, shard_sizes: $shard_sizes, programs: $programs}')

# Run the workflow with the list of workloads.
echo $WORKLOADS | gh workflow run suite.yml --ref $GIT_REF --json
