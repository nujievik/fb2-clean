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
fn iter_file() {
    ITERABLE.iter().for_each(|&(ty, f)| {
        let f = InputFile { ty, path: data(f) };
        let i = Input::File(f.clone());

        let mut iter = i.iter();
        assert_eq!(Some(f), iter.next());
        assert_eq!(None, iter.next());
    })
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
