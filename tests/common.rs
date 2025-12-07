use fb2_clean::*;
use std::path::{Path, PathBuf};
use clap::Parser;

#[cfg(not(target_os = "macos"))]
pub static ITERABLE: [(InputFileType, &str); 5] = [
    (InputFileType::Fb2Zip, "book.fb2.zip"),
    (InputFileType::Fb2, "dummy.fb2"),
    (InputFileType::Fb2Zip, "dummy.fb2.zip"),
    (InputFileType::Fb2, "dummy.FB2"),
    (InputFileType::Fb2Zip, "dummy.FB2.ZIP"),
];

#[cfg(target_os = "macos")]
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
    p.push(s);
    p.into()
}

pub fn temp(s: &str) -> Box<Path> {
    let mut p = PathBuf::from(data(""));
    p.push("temp");
    p.push(s);
    p.into()
}
