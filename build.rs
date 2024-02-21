use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let dir = env::var_os("CARGO_MANIFEST_DIR").ok_or(env::VarError::NotPresent)?;

    let mut script = PathBuf::from(dir.clone());
    script.push("linker.ld");

    println!("cargo:rustc-link-arg-bins={}", script.display());
    println!("cargo:rerun-if-changed={}", script.display());

    let mut asm = PathBuf::from(dir);
    asm.push("src/boot/boot.S");

    println!("cargo:rerun-if-changed={}", asm.display());

    Ok(())
}
