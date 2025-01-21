use std::fmt::{Display, Formatter};

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
#[clap(rename_all = "kebab_case")]
pub enum ProgramId {
    // Loop.
    Loop10k,
    Loop100k,
    Loop1m,
    Loop3m,
    Loop10m,
    Loop30m,
    Loop100m,
    Loop300m,

    // Fibonacci.
    Fibonacci20k,
    Fibonacci200k,
    Fibonacci2m,
    Fibonacci4m,
    Fibonacci20m,
    Fibonacci40m,
    Fibonacci200m,
    Fibonacci400m,
    Fibonacci1b,
    Fibonacci2b,
    Fibonacci4b,

    // SHA-256.
    Sha256100kb,
    Sha256300kb,
    Sha2561mb,
    Sha2563mb,
    Sha25610mb,

    // Keccak-256.
    Keccak256100kb,
    Keccak256300kb,
    Keccak2561mb,
    Keccak2563mb,
    Keccak25610mb,

    // SSZ Withdrawals.
    SSZWithdrawals,

    // Tendermint.
    Tendermint,
    
    // RSP
    Rsp20526626,
    Rsp20526627,
    Rsp20526628,
    Rsp20526629,
    Rsp20526630,
    Rsp20528708,
    Rsp20528709,
    Rsp20528710,
    Rsp20528711,
    Rsp20528712,
    
    // Signatures
    ECDSAVerify,    
    EDDSAVerify,

    Helios,

    Groth16ProofVerify,

    ZKEmail,
}

impl ProgramId {
    /// The "priority" of a program is used to sort the programs in the performance report.
    ///
    /// The higher the priority, the more work the proof requires. 
    pub(crate) fn priority(&self) -> usize {
        match self {
            // Loop
            ProgramId::Loop10k => 1,
            ProgramId::Loop100k => 2,
            ProgramId::Loop1m => 3,
            ProgramId::Loop3m => 4,
            ProgramId::Loop10m => 5,
            ProgramId::Loop30m => 6,
            ProgramId::Loop100m => 7,
            ProgramId::Loop300m => 8,
            
            // Fibonacci
            ProgramId::Fibonacci20k => 1,
            ProgramId::Fibonacci200k => 2,
            ProgramId::Fibonacci2m => 3,
            ProgramId::Fibonacci4m => 4,
            ProgramId::Fibonacci20m => 5,
            ProgramId::Fibonacci40m => 6,
            ProgramId::Fibonacci200m => 7,
            ProgramId::Fibonacci400m => 8,
            ProgramId::Fibonacci1b => 9,
            ProgramId::Fibonacci2b => 10,
            ProgramId::Fibonacci4b => 11,
            
            // SHA-256
            ProgramId::Sha256100kb => 1,
            ProgramId::Sha256300kb => 2,
            ProgramId::Sha2561mb => 3,
            ProgramId::Sha2563mb => 4,
            ProgramId::Sha25610mb => 5,
            
            // Keccak-256
            ProgramId::Keccak256100kb => 1,
            ProgramId::Keccak256300kb => 2,
            ProgramId::Keccak2561mb => 3,
            ProgramId::Keccak2563mb => 4,
            ProgramId::Keccak25610mb => 5,
            
            // SSZ Withdrawals
            ProgramId::SSZWithdrawals => 1,
            
            // Tendermint
            ProgramId::Tendermint => 1,
            
            // RSP
            ProgramId::Rsp20526626 => 1,
            ProgramId::Rsp20526627 => 1,
            ProgramId::Rsp20526628 => 1,
            ProgramId::Rsp20526629 => 1,
            ProgramId::Rsp20526630 => 1,
            ProgramId::Rsp20528708 => 1,
            ProgramId::Rsp20528709 => 1,
            ProgramId::Rsp20528710 => 1,
            ProgramId::Rsp20528711 => 1,
            ProgramId::Rsp20528712 => 1,
            
            // Signatures
            ProgramId::ECDSAVerify => 1,
            ProgramId::EDDSAVerify => 1,

            ProgramId::Helios => 1,

            ProgramId::Groth16ProofVerify => 1,

            ProgramId::ZKEmail => 1,
        }
    }
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
            ProgramId::Loop10k => write!(f, "loop-10k"),
            ProgramId::Loop100k => write!(f, "loop-100k"),
            ProgramId::Loop1m => write!(f, "loop-1m"),
            ProgramId::Loop3m => write!(f, "loop-3m"),
            ProgramId::Loop10m => write!(f, "loop-10m"),
            ProgramId::Loop30m => write!(f, "loop-30m"),
            ProgramId::Loop100m => write!(f, "loop-100m"),
            ProgramId::Loop300m => write!(f, "loop-300m"),
            ProgramId::Fibonacci20k => write!(f, "fibonacci-20k"),
            ProgramId::Fibonacci200k => write!(f, "fibonacci-200k"),
            ProgramId::Fibonacci2m => write!(f, "fibonacci-2m"),
            ProgramId::Fibonacci4m => write!(f, "fibonacci-4m"),
            ProgramId::Fibonacci20m => write!(f, "fibonacci-20m"),
            ProgramId::Fibonacci40m => write!(f, "fibonacci-40m"),
            ProgramId::Fibonacci200m => write!(f, "fibonacci-200m"),
            ProgramId::Fibonacci400m => write!(f, "fibonacci-400m"),
            ProgramId::Fibonacci1b => write!(f, "fibonacci-1b"),
            ProgramId::Fibonacci2b => write!(f, "fibonacci-2b"),
            ProgramId::Fibonacci4b => write!(f, "fibonacci-4b"),
            ProgramId::Sha256100kb => write!(f, "sha256-100kb"),
            ProgramId::Sha256300kb => write!(f, "sha256-300kb"),
            ProgramId::Sha2561mb => write!(f, "sha256-1mb"),
            ProgramId::Sha2563mb => write!(f, "sha256-3mb"),
            ProgramId::Sha25610mb => write!(f, "sha256-10mb"),
            ProgramId::Keccak256100kb => write!(f, "keccak256-100kb"),
            ProgramId::Keccak256300kb => write!(f, "keccak256-300kb"),
            ProgramId::Keccak2561mb => write!(f, "keccak256-1mb"),
            ProgramId::Keccak2563mb => write!(f, "keccak256-3mb"),
            ProgramId::Keccak25610mb => write!(f, "keccak256-10mb"),
            ProgramId::SSZWithdrawals => write!(f, "ssz-withdrawals"),
            ProgramId::Tendermint => write!(f, "tendermint"),
            ProgramId::Rsp20526626 => write!(f, "rsp-20526626"),
            ProgramId::Rsp20526627 => write!(f, "rsp-20526627"),
            ProgramId::Rsp20526628 => write!(f, "rsp-20526628"),
            ProgramId::Rsp20526629 => write!(f, "rsp-20526629"),
            ProgramId::Rsp20526630 => write!(f, "rsp-20526630"),
            ProgramId::Rsp20528708 => write!(f, "rsp-20528708"),
            ProgramId::Rsp20528709 => write!(f, "rsp-20528709"),
            ProgramId::Rsp20528710 => write!(f, "rsp-20528710"),
            ProgramId::Rsp20528711 => write!(f, "rsp-20528711"),
            ProgramId::Rsp20528712 => write!(f, "rsp-20528712"),
            ProgramId::ECDSAVerify => write!(f, "ecdsa-verify"),
            ProgramId::EDDSAVerify => write!(f, "eddsa-verify"),
            ProgramId::Helios => write!(f, "helios"),
            ProgramId::Groth16ProofVerify => write!(f, "groth16-proof-verify"),
            ProgramId::ZKEmail => write!(f, "zk-email"),
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
