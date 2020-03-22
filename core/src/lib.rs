use std::path::PathBuf;

pub mod error;
pub mod gui;

pub fn get_asset_path() -> PathBuf {
    find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap()
}
