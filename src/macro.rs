#[macro_export]
macro_rules! cli_version {
    ($version:literal) => {
        concat!("clrun ", $version)
    };
}

#[macro_export]
macro_rules! error_message {
    ($msg:literal) => {
        concat!("\x1b[1;31merror:\x1b[0m ", $msg)
    };
}

#[macro_export]
macro_rules! println_unexpected_arg {
    ($arg:expr) => {
        println!(
            "\x1b[1;31merror:\x1b[0m unexpected argument \x1b[1;33m'{}'\x1b[0m found\n\n{}",
            $arg, ERROR_USAGE_MESSAGE
        );
    };
}
