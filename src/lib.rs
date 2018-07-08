extern crate rand;

use rand::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

pub mod cmd;
pub mod printer;

pub fn pick_file(path: &Path) -> PathBuf {
    let mut rng = thread_rng();
    let contents: Vec<PathBuf> = fs::read_dir(path)
        .expect("failed to read path dir")
        .map(|r| r.expect("failed to read path").path())
        .collect();
    if contents.len() == 0 {
        eprint!("Someones a chicken... Path points to dir with nothing!\n");
        exit(0);
    }
    let unlucky = rng.gen_range(0, contents.len());

    contents[unlucky].clone()
}

pub fn fire(path: &Path, chambers: usize) -> bool {
    let mut rng = thread_rng();
    if rng.gen_range(0, chambers) == 0 {
        if fs::metadata(path)
            .expect("failed to read file meta")
            .is_dir()
        {
            std::fs::remove_dir_all(path).expect("failed to delete dir");
        } else {
            std::fs::remove_file(path).expect("failed to delete file");
        }

        return true;
    }
    false
}
