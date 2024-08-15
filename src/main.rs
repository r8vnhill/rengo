mod asm;
mod ast;
mod compiler;

use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let program = read_program(&args[1])?;
    let assembly = compile(program);

    let asm_output_path = create_output_paths("build/s/", "out.asm")?;
    let obj_output_path = create_output_paths("build/obj/", "out.obj")?;
    let exe_output_path = create_output_paths("build/", "out.exe")?;

    write_assembly(&asm_output_path, &assembly)?;
    assemble(&asm_output_path, &obj_output_path)?;
    link(&obj_output_path, &exe_output_path)?;

    Ok(())
}

fn parse_args() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <program>", args[0]);
        return Err("Invalid number of arguments".into());
    }
    Ok(args)
}

fn read_program(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = io::BufReader::new(input_file);
    let input_program = reader.lines().next().ok_or("Error: empty input file")??;
    let program = input_program.parse::<i64>().map_err(|_| "Error: invalid program")?;
    Ok(program)
}

fn create_output_paths(dir: &str, file_name: &str) -> Result<PathBuf, io::Error> {
    let output_dir = Path::new(dir);
    fs::create_dir_all(output_dir)?;
    Ok(output_dir.join(file_name))
}

fn write_assembly(asm_output_path: &Path, assembly: &str) -> Result<(), io::Error> {
    fs::write(asm_output_path, assembly).expect("Failed to write assembly file");
    Ok(())
}

fn assemble(asm_output_path: &Path, obj_output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

fn link(obj_output_path: &Path, exe_output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("clang");

    if std::env::consts::OS == "windows" {
        // Use the correct format for passing the /subsystem:console option to the MSVC linker
        cmd.args(&[
            "-g",
            "-m64",
            "-o",
            exe_output_path.to_str().unwrap(),
            "src/main.c",
            obj_output_path.to_str().unwrap(),
            "-Xlinker", "/subsystem:console",
        ]);
    } else {
        cmd.args(&[
            "-g",
            "-m64",
            "-o",
            exe_output_path.to_str().unwrap(),
            "src/main.c",
            obj_output_path.to_str().unwrap(),
        ]);
    }

    cmd.status().expect("Failed to link .obj to .exe");

    Ok(())
}

fn compile(program: i64) -> String {
    format!(
        "
section .text
global _start
_start:
  mov RAX, {}
  ret\n",
        program
    )
}
