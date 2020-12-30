use std::{
    env,
    error::Error,
    ffi::{OsStr, OsString},
    path::Path,
};

pub type Result = std::result::Result<i32, Box<dyn Error>>;

#[macro_export]
macro_rules! main {
    ($name:expr) => {
        fn main() -> ! {
            let name = $crate::name($name);

            std::process::exit(match program(&name) {
                Err(error) => {
                    eprintln!("{}: {}", name, error);
                    1
                }
                Ok(code) => code,
            });
        }
    };
}

pub fn args() -> Vec<String> {
    env::args_os()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

pub fn args_os() -> Vec<OsString> {
    env::args_os().collect()
}

pub fn name(default: &str) -> String {
    match env::args_os().next() {
        None => String::from(default),
        Some(os_string) => match Path::new(&os_string).file_stem() {
            None => String::from(default),
            Some(os_str) => os_str.to_string_lossy().into_owned(),
        },
    }
}

pub fn name_os(default: &OsStr) -> OsString {
    match env::args_os().next() {
        None => OsString::from(default),
        Some(os_string) => match Path::new(&os_string).file_stem() {
            None => OsString::from(default),
            Some(os_str) => os_str.to_os_string(),
        },
    }
}
