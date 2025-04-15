use std::path::PathBuf;

pub enum SortType {
    DirsFirst,
    FilesFirst,
}

pub fn sort_paths_by_type(sort_type: SortType, paths: &mut Vec<PathBuf>) -> () {
    match sort_type {
        SortType::DirsFirst => {
            paths.sort_by(|a, b| b.is_dir().cmp(&a.is_dir()));
        }
        SortType::FilesFirst => {
            paths.sort_by(|a, b| b.is_file().cmp(&a.is_file()));
        }
    }
}
