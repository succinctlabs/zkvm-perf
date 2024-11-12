use std::fmt::{Display, Formatter};

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
#[clap(rename_all = "kebab_case")]
pub enum ProgramId {
    Loop10k,
    Loop100k,
    Loop1m,
    Loop3m,
    Loop10m,
    Loop30m,
    Loop100m,
    Sha2561mb,
    Sha25610mb,
    Keccak2561mb,
    Keccak25610mb,
    Fibonacci,
    SSZWithdrawals,
    Tendermint,
    Sha2Chain,
    Reth,
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum ProverId {
    Risc0,
    SP1,
}

/// An identifier used to select the hash function to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum HashFnId {
    Sha256,
    Poseidon,
    Blake3,
    Keccak256,
}

impl Display for ProgramId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramId::Loop10k => write!(f, "loop10k"),
            ProgramId::Loop100k => write!(f, "loop100k"),
            ProgramId::Loop1m => write!(f, "loop1m"),
            ProgramId::Loop3m => write!(f, "loop3m"),
            ProgramId::Loop10m => write!(f, "loop10m"),
            ProgramId::Loop30m => write!(f, "loop30m"),
            ProgramId::Loop100m => write!(f, "loop100m"),
            ProgramId::Sha2561mb => write!(f, "sha256-1mb"),
            ProgramId::Sha25610mb => write!(f, "sha256-10mb"),
            ProgramId::Keccak2561mb => write!(f, "keccak256-1mb"),
            ProgramId::Keccak25610mb => write!(f, "keccak256-10mb"),
            ProgramId::Fibonacci => write!(f, "fibonacci"),
            ProgramId::SSZWithdrawals => write!(f, "ssz-withdrawals"),
            ProgramId::Tendermint => write!(f, "tendermint"),
            ProgramId::Sha2Chain => write!(f, "sha2-chain"),
            ProgramId::Reth => write!(f, "reth"),
        }
    }
}

impl Display for ProverId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProverId::Risc0 => write!(f, "risc0"),
            ProverId::SP1 => write!(f, "sp1"),
        }
    }
}

impl Display for HashFnId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashFnId::Sha256 => write!(f, "sha-256"),
            HashFnId::Poseidon => write!(f, "poseidon"),
            HashFnId::Blake3 => write!(f, "blake3"),
            HashFnId::Keccak256 => write!(f, "keccak256"),
        }
    }
}
