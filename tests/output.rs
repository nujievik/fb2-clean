#[allow(unused)]
mod common;

use common::*;
use fb2_clean::*;
use std::fs;

#[test]
fn create_and_remove_dirs() {
    let dir = temp("create_and_remove_dirs");
    let mut o = Output {
        dir: dir.clone(),
        len_created_dir_chain: 0,
    };
    let _ = fs::remove_dir_all(&dir);

    o.create_dirs().unwrap();
    assert!(dir.exists());
    assert_eq!(o.len_created_dir_chain, 1);

    o.remove_created_dirs();
    assert!(!dir.exists());
}
