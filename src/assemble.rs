use std::path::Path;
use std::process::Command;

pub(crate) fn assemble(asm_output_path: &Path, obj_output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let format = match std::env::consts::OS {
        "windows" => "win64",
        "linux" => "elf64",
        "macos" => "macho64",
        _ => return Err("Unsupported operating system".into()),
    };

    Command::new("nasm")
        .args(&["-f", format, asm_output_path.to_str().unwrap(), "-o", obj_output_path.to_str().unwrap()])
        .status()
        .expect("Failed to assemble .asm to .obj");

    Ok(())
}
