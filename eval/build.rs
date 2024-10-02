use std::fs;
use std::path::{Path};
use std::process::Command;

fn needs_rebuild(program_dir: &Path, elf_path: &Path) -> bool {
    if !elf_path.exists() {
        return true;
    }

    let elf_modified = fs::metadata(elf_path)
        .and_then(|m| m.modified())
        .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);

    fs::metadata(program_dir)
        .and_then(|m| m.modified())
        .map_or(true, |dir_modified| dir_modified > elf_modified)
}

fn main() {
    vergen::EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .emit()
        .expect("Failed to generate vergen information");

    let programs_dir = Path::new("../programs");

    if let Ok(entries) = fs::read_dir(programs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let program_name = path.file_name().unwrap().to_str().unwrap();

                let target = if program_name.ends_with("-risc0") {
                    "riscv32im-risc0-zkvm-elf"
                } else {
                    "riscv32im-succinct-zkvm-elf"
                };

                let elf_dir = path.join("elf");
                let elf_path = elf_dir.join(target);

                if !needs_rebuild(&path, &elf_path) {
                    continue;
                }

                fs::create_dir_all(&elf_dir).expect("Failed to create elf directory");

                let status = Command::new("cargo")
                    .current_dir(&path)
                    .args(&[
                        "build",
                        "--release",
                        "--target", target,
                        "--out-dir", elf_dir.to_str().unwrap(),
                    ])
                    .status()
                    .expect("Failed to execute cargo build");

                if !status.success() {
                    panic!("Failed to build program: {}", program_name);
                }
            }
        }
    }
}
