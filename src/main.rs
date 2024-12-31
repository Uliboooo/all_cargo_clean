use clap::Parser;
use std::{
    fmt::Display, fs::{self, ReadDir}, path::PathBuf, process
};

#[derive(Parser)]
struct Args {
    /// path
    dir_path: String,
}

#[derive(Debug)]
enum ACCError {
    //   All Cargo Clean(ACC)
    FailedGetReadDir,
    FailedConvertPathToStr,
    // Cmd(io::Error),
    FailedRunCommand,
    NoCargoDir,
}

impl Display for ACCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ACCError::FailedGetReadDir => write!(f, "failed get readdir"),
            ACCError::FailedConvertPathToStr => write!(f, "failed convert path to str"),
            // ACCError::Cmd(error) => todo!(),
            ACCError::FailedRunCommand => write!(f, "failed run command"),
            ACCError::NoCargoDir => write!(f, "no cargo dir"),
        }
    }
}

/// - return PathBuf of Cargo.toml if directory include Cargo.toml.
/// - if other times(don't fonund Cargo.toml & more Error), retrun ACCError.
fn is_cargo_dir(dir: &PathBuf) -> Result<PathBuf, ACCError> {
    let f_list = fs::read_dir(dir).map_err(|_| ACCError::FailedGetReadDir)?;
    for file in f_list {
        let f = file.map_err(|_| ACCError::FailedGetReadDir)?;
        if f.file_name() == "Cargo.toml" {
            return Ok(f.path());
        }
    }
    Err(ACCError::NoCargoDir)
}

fn clean(toml_path: PathBuf) -> Result<bool, ACCError> {
    let r = process::Command::new("cargo")
        .args([
            "clean",
            "--manifest-path",
            toml_path.to_str().ok_or(ACCError::FailedConvertPathToStr)?,
        ])
        .output();

    if r.map_err(|_| ACCError::FailedRunCommand)?.status.success() {
        Ok(true)
    } else {
        Err(ACCError::FailedRunCommand)
    }
}

fn search_and_destroy(dir_list: ReadDir) -> Result<(), ACCError> {
    for folder in dir_list {
        let folder_path = folder.unwrap().path();
        if folder_path.is_dir() {
            let is_cargo_dir = is_cargo_dir(&folder_path);
            match is_cargo_dir {
                Ok(p) => {
                    if clean(p).is_err() {
                        continue;
                    };
                    return Ok(());
                }
                Err(_) => {
                    if let Ok(r) = fs::read_dir(folder_path) {
                        if search_and_destroy(r).is_err() {
                            continue;
                        };
                    } else {
                        continue;
                    }
                }
            }
        } else {
            continue;
        }
    }
    Ok(())
}

fn main() {
    let path = PathBuf::from(Args::parse().dir_path);
    if path.is_dir() {
        match search_and_destroy(fs::read_dir(path).unwrap()) {
            Ok(_) => println!("sucess"),
            Err(e) => println!("{}", e),
        };
    } else {
        println!("this path is not dir.");
    }
}
