use std::{
    env::Args,
    path::{Path, PathBuf},
};

pub fn args_to_path(argc: usize, argv: &mut Args) -> PathBuf {
    if argc == 1 {
        match std::env::current_dir() {
            Ok(path_buf) => return path_buf,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
    }

    let path_option = &argv.nth(1);
    match path_option {
        Some(path_str) => {
            let path = Path::new(&path_str).to_path_buf();
            if path.exists() {
                return path;
            } else {
                eprintln!("Path `{}` does not exist", path.display());
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Unable to parse command-line input.");
            std::process::exit(1);
        }
    }
}
