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
        created_dirs: Vec::new(),
    };
    let _ = fs::remove_dir_all(&dir);

    o.create_dirs().unwrap();
    assert!(dir.exists());
    assert_eq!(&o.created_dirs, &vec![dir.clone()]);

    o.remove_created_dirs();
    assert!(!dir.exists())
}
