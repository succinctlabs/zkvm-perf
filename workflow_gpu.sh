#! /bin/bash
INSTANCES=(
    "aws-g6-xlarge"
    "aws-g6-2xlarge"
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
    "loop30m"
    "loop100m"
    "loop300m"
    "fibonacci20k"
    "fibonacci200k"
    "fibonacci2m"
    "fibonacci4m"
    "fibonacci20m"
    "fibonacci40m"
    "fibonacci200m"
    "fibonacci400m"
    "sha256100kb"
    "sha256300kb"
    "sha2561mb"
    "sha2563mb"
    "keccak256100kb"
    "keccak256300kb"
    "keccak2561mb"
    "keccak2563mb"
    "ssz-withdrawals"
    "tendermint"
    "rsp20526626"
    "rsp20526627"
    "rsp20526628"
    "rsp20526629"
    "rsp20526630"
    "rsp20528708"
    "rsp20528709"
    "rsp20528710"
    "rsp20528711"
    "rsp20528712"
    # "ecdsa-verify"
    "eddsa-verify"
    "helios"
    "groth16-proof-verify"
    "zk-email"
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