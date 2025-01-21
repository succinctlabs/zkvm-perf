#! /bin/bash

INSTANCES=(
    # "aws-m7i-xlarge"
    "aws-m7i-2xlarge"
    # "aws-m7i-4xlarge"
    # "aws-m7i-8xlarge"
    "aws-m7i-16xlarge"
)
PROVERS=(
    "sp1"
    "risc0"
)
SHARD_SIZES=(
    "21"
)
PROGRAMS=(
    "loop10k"
    "loop100k"
    "loop1m"
    "loop3m"
    "loop10m"
    "fibonacci20k"
    "fibonacci200k"
    "fibonacci2m"
    "fibonacci4m"
    "fibonacci20m"
    "sha256300kb"
    "keccak256300kb"
    "ssz-withdrawals"
    "tendermint"
    # "ecdsa-verify"
    "eddsa-verify"
    "rsp20526626"
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