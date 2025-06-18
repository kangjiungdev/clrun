mod r#macro;

use std::path::PathBuf;
use std::process::{self, Command};
use std::{env, fs};

const VERSION: &str = concat!("clrun ", env!("CARGO_PKG_VERSION"));

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
                    eprintln_unexpected_arg!(s);
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
                    eprintln_unexpected_arg!(s);
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

struct Error {
    error: &'static str,
}

impl Error {
    const fn new(error: &'static str) -> Error {
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
        // Os str인가 그런거 있다더라. 나중에 그걸로 바꿔서 UTF-8 아니어도 작동하게 해야겠다. 파일 이름 && 경로가 일반적인 OS면 무조건 UTF-8 이라 에러 생길일은 없겠다만.
        eprintln!(error_message!("Path is not valid UTF-8"));
        std::process::exit(1);
    };

    let status = Command::new(compiler)
        .arg(file)
        .arg("-o")
        .arg(binary_file_path)
        .status();

    // if문 너무 많아. 끔찍해
    if let Ok(status) = status {
        if status.success() {
            let run_binary_file = Command::new(binary_file_path).status();

            let mut exit_code: u8 = 0;

            if let Ok(status) = run_binary_file {
                if !status.success() {
                    exit_code = 1;
                }
            } else {
                eprintln!(error_message!("Failed to run binary file"));
                std::process::exit(1);
            }

            let success = fs::remove_file(binary_file_path);

            if success.is_err() {
                eprintln!(error_message!("Failed to delete binary file"));
                std::process::exit(1);
            }
            /* 이렇게 해야지 실행시키고 삭제했던 바이너리 파일 exit status에 맞춰서 이 프로그램 exit code 정할 수 있음.
            바이너리 파일 exit status 0 아니라고 바로 프로그램 exit 해버리면 파일 삭제 못하니까.
            if !status.success() 에 삭제+exit1 코드를 넣거나 핸들러 함수로 분리해도 되긴 함.
            if문에 넣는건 삭제 코드 또 한번 더 적게 되는거니까 비효율적.
            핸들러 함수 사용하는건 어차피 두 번만 사용할건데 만드는건 오버 스탠스 + 핸들러 함수 인자가 사용할 스택 메모리를 따로 할당하기에
            (인자로 받은 표현식의 값을 스택에 push하고 그 메모리에 접근(최적화 안되었을 때 방식. 변수를 인자에 경우 똑같은 데이터가 스택에 2번 저장되어 비효율적.)하는게 아니라
            인자로 받는 표현식의 값이 저장된 스택 메모리(=호출 시 인자로 넣은 변수가 사용하는 스택 메모리)를 재사용하도록 LLVM이 최적화 할 가능성 있음) 비효율적이라 판단.
            결론적으로 변수로 exit code 지정하는게 딱 한번만 적는거라 제일 좋다고 판단함. LLVM 최적화 때문에 이 방식이 핸들러 함수 사용보다 성능이 안 좋을 가능성도 있음. */
            // 결론적으로 귀찮아서 핸들러 함수 안만듬.
            // 나중에 기능 확장하게 되면 어쩔 수 없이 핸들러 함수 만들 수도 있긴 함.
            // 바이너리 파일 실패 => 이 프로그램 exit code 1
            // 바이너리 파일 성공 => 이 프로그램 exit code 0
            process::exit(exit_code as i32);
        } else {
            eprintln!(error_message!("Compile failed"));
            std::process::exit(1);
        }
    } else {
        eprintln!("{} {}", error_message!("Failed to run"), compiler);
        std::process::exit(1);
    }
}

fn get_base_path() -> PathBuf {
    let home_path = dirs::home_dir();
    if let Some(home_path) = home_path {
        let clrun_path = home_path.join(".clrun").join("build");
        let success = fs::create_dir_all(&clrun_path);
        if let Err(_) = success {
            eprintln!(error_message!("Failed to create .clrun directory"));
            process::exit(1);
        }
        clrun_path
    } else {
        eprintln!(error_message!("No home directory"));
        process::exit(1);
    }
}
