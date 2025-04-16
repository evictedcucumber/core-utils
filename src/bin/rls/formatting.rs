fn format_dir(directory: &std::path::PathBuf) -> String {
    assert!(directory.is_dir());

    match directory.file_name() {
        Some(diro) => match diro.to_str() {
            Some(dirname) => format!("\x1b[1;34m{}\x1b[0m/", dirname),
            None => {
                eprintln!(
                    "Unable to convert OsStr to str for path `{}`",
                    directory.display()
                );
                String::from("")
            }
        },
        None => {
            eprintln!("Unable to get filename from path `{}`", directory.display());
            String::from("")
        }
    }
}

fn format_file(file: &std::path::PathBuf) -> String {
    assert!(file.is_file());

    match file.file_name() {
        Some(fo) => match fo.to_str() {
            Some(filename) => format!("{}", filename),
            None => {
                eprintln!(
                    "Unable to convert OsStr to str for path `{}`",
                    file.display()
                );
                String::from("")
            }
        },
        None => {
            eprintln!("Unable to get filename from path `{}`", file.display());
            String::from("")
        }
    }
}

pub fn format_path(path: &std::path::PathBuf) -> String {
    if path.is_dir() {
        return format_dir(path);
    } else if path.is_file() {
        return format_file(path);
    } else {
        return String::from(path.to_str().unwrap_or_default());
    }
}
