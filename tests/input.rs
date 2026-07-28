#[allow(unused)]
mod common;

use common::*;
use fb2_clean::*;
use std::collections::HashSet;

#[test]
fn iter_dir() {
    let i = Input::Dir(data(""));
    let exp: HashSet<_> = ITERABLE
        .iter()
        .map(|&(ty, f)| InputFile { ty, path: data(f) })
        .collect();

    assert_eq!(exp, i.iter().collect());
}

#[test]
fn iter_files() {
    let mut files: Vec<InputFile> = Vec::new();

    ITERABLE.iter().for_each(|&(ty, f)| {
        let f = InputFile { ty, path: data(f) };
        files.push(f);
    });

    let mut it = Input::Files(files).iter();

    ITERABLE.iter().for_each(|&(ty, f)| {
        assert_eq!(it.next().unwrap(), InputFile { ty, path: data(f) });
    });

    assert!(it.next().is_none());
}

#[test]
fn iter_dir_upper() {
    let i = Input::Dir(data("upper_case"));
    let mut iter = i.iter();
    for _ in 0..2 {
        assert!(iter.next().is_some());
    }
    assert!(iter.next().is_none());
}
