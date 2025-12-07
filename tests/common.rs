use clap::Parser;
use fb2_clean::*;
use std::path::{Path, PathBuf};

pub static ITERABLE: [(InputFileType, &str); 3] = [
    (InputFileType::Fb2Zip, "book.fb2.zip"),
    (InputFileType::Fb2, "dummy.fb2"),
    (InputFileType::Fb2Zip, "dummy.fb2.zip"),
];

pub fn cfg(args: &[&str]) -> Config {
    let mut xs = vec!["x"]; // simular binary name
    xs.extend_from_slice(args);
    Config::try_parse_from(xs).unwrap()
}

pub fn data(s: &str) -> Box<Path> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    for s in s.split('/') {
        p.push(s);
    }
    p.into()
}

pub fn temp(s: &str) -> Box<Path> {
    let mut p = PathBuf::from(data(""));
    p.push("temp");
    for s in s.split('/') {
        p.push(s);
    }
    p.into()
}
