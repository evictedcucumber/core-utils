use std::path::PathBuf;

mod argparse;
mod formatting;
mod handlers;
mod sorting;

pub const EXE_NAME: &str = env!("CARGO_CRATE_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const REPO: &str = env!("CARGO_PKG_REPOSITORY");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let help_text =
        std::fs::read_to_string("docs/help.txt").expect("Should be able to read help file");
    let mut argv = std::env::args();
    let argc = argv.len();

    let args = argparse::parse_args(argc, &argv);
    for arg in args {
        use argparse::ArgumentType;
        match arg {
            ArgumentType::Help => {
                println!("{}", help_text);
            }
            ArgumentType::Version => {
                println!("{} - {}\nv{}\n{}", EXE_NAME, DESCRIPTION, VERSION, REPO);
            }
        }
    }

    let working_dir = handlers::args_to_path(argc, &mut argv);
    dbg!(&working_dir);

    let mut paths: Vec<PathBuf> = std::fs::read_dir(&working_dir)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();

    sorting::sort_paths_by_type(sorting::SortType::DirsFirst, &mut paths);

    // let stripped_paths = format::util::strip_all_cwd(&working_dir, &paths);

    for path in &paths {
        println!("{}", formatting::format_path(path));
    }
}
