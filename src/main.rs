mod r#macro;

use std::path::PathBuf;
use std::process::{self, Command};
use std::{env, fs};

const VERSION: &str = cli_version!("0.2.0"); // Version of Cli

enum Language {
    C,
    Cpp,
}

const HELP_MESSAGE: &str = "Clang/Clang++ Runner

Usage:
  clrun [options]
  clrun <language> <filename>

Languages:
  c         Compile and run as a C program
  cpp       Compile and run as a C++ program
  c++       Alias for cpp

Options:
  -h, --help       Show this help message
  -v, --version    Show version information";

pub const ERROR_USAGE_MESSAGE: &str = "Usage:\n  clrun [options]\n  clrun <language> <filename>\n\nFor more information, try '--help'.";

fn main() {
    let mut args = env::args();
    args.next();

    let compiler = get_args(args.next(), None);

    let compiler = match compiler.as_str() {
        s if s.starts_with("--") => {
            match s {
                "--help" => println!("{HELP_MESSAGE}"),
                "--version" => println!("{VERSION}"),
                _ => {
                    println_unexpected_arg!(s);
                    process::exit(1);
                }
            }
            process::exit(0);
        }
        s if s.starts_with("-") => {
            match s {
                "-h" => println!("{HELP_MESSAGE}"),
                "-v" => println!("{VERSION}"),
                _ => {
                    println_unexpected_arg!(s);
                    process::exit(1);
                }
            }
            process::exit(0);
        }
        "c" => Language::C,
        "c++" | "cpp" => Language::Cpp,
        _ => {
            eprintln!(
                "{} {}\n\n{}",
                error_message!("Unsupported language:"),
                compiler,
                ERROR_USAGE_MESSAGE
            );
            process::exit(1);
        }
    };

    let file = get_args(
        args.next(),
        Some(Error::new(error_message!("No input file"))),
    );
    run_command(compiler, &file);
}

struct Error<'a> {
    error: &'a str,
}

impl Error<'_> {
    fn new(error: &'_ str) -> Error {
        return Error { error };
    }
}

fn get_args(args: Option<String>, err: Option<Error>) -> String {
    match args {
        Some(arg) => {
            if arg.is_empty() {
                if let Some(err) = err {
                    eprintln!("{}\n\n{}", err.error, ERROR_USAGE_MESSAGE);
                } else {
                    println!("{HELP_MESSAGE}");
                }
                std::process::exit(1);
            } else {
                arg
            }
        }
        None => {
            if let Some(err) = err {
                eprintln!("{}\n\n{}", err.error, ERROR_USAGE_MESSAGE);
                std::process::exit(1);
            } else {
                println!("{HELP_MESSAGE}");
                std::process::exit(0);
            }
        }
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
        eprintln!(error_message!("Path is not valid UTF-8"));
        std::process::exit(1);
    };

    let status = Command::new(compiler)
        .arg(file)
        .arg("-o")
        .arg(binary_file_path)
        .status();

    if let Ok(status) = status {
        if status.success() {
            let _ = Command::new(binary_file_path)
                .status()
                .expect(error_message!("Failed to run binary file"));
            let _ = Command::new("rm")
                .arg(binary_file_path)
                .status()
                .expect(error_message!("Failed to delete binary file"));
        } else {
            eprintln!("{}", error_message!("Compile failed"));
            std::process::exit(1);
        }
    } else {
        println!("{} {}", error_message!("Failed to run"), compiler);
    }
}

fn get_base_path() -> PathBuf {
    let clrun_path = dirs::home_dir()
        .expect(error_message!("No home directory"))
        .join(".clrun")
        .join("build");
    fs::create_dir_all(&clrun_path).expect(error_message!("Failed to create .clrun directory"));
    clrun_path
}
