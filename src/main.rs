use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

enum Language {
    C,
    Cpp,
}

const ERROR: &str = "clrun: \x1b[1;31merror:\x1b[0m";

fn main() {
    let mut args = env::args();
    args.next();

    if let Some(compiler) = args.next() {
        let compiler = match compiler.as_str() {
            "c" => Language::C,
            "c++" | "cpp" => Language::Cpp,
            _ => {
                eprintln!("{ERROR} Unsupported language: {compiler}");
                std::process::exit(1);
            }
        };
        if let Some(file) = &args.next() {
            if !file.is_empty() {
                run_command(compiler, &file);
            } else {
                eprintln!("{ERROR} No input file");
                std::process::exit(1);
            }
        } else {
            eprintln!("{ERROR} No input file");
            std::process::exit(1);
        }
    } else {
        println!(
            "Usage: clrun <language> <filename>\n\n<language> options:\nc         Compile and run as C program\nc++/cpp   Compile and run as C++ program"
        );
    }
}

fn run_command(compiler: Language, file: &str) {
    let compiler = match compiler {
        Language::C => "clang",
        Language::Cpp => "clang++",
    };
    let binary_file_path: PathBuf = get_base_path().join("main").into();
    let binary_file_path = if let Some(s) = binary_file_path.to_str() {
        s
    } else {
        eprintln!("{ERROR} Path is not valid UTF-8");
        std::process::exit(1);
    };

    let status = Command::new(compiler)
        .arg(file)
        .arg("-o")
        .arg(binary_file_path)
        .status()
        .expect(&format!("{ERROR} Failed to run {compiler}"));

    if status.success() {
        let _ = Command::new(binary_file_path)
            .status()
            .expect(&format!("{ERROR} Failed to run binary file"));
        let _ = Command::new("rm")
            .arg(binary_file_path)
            .status()
            .expect(&format!("{ERROR} Failed to delete binary file"));
    } else {
        eprintln!("{ERROR} Compile failed");
        std::process::exit(1);
    }
}

fn get_base_path() -> PathBuf {
    let clrun_path = dirs::home_dir()
        .expect(&format!("{ERROR} No home directory"))
        .join(".clrun")
        .join("build");
    fs::create_dir_all(&clrun_path).expect(&format!("{ERROR} Failed to create .clrun directory"));
    clrun_path
}
