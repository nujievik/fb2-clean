#[allow(unused)]
mod common;

use common::*;
use fb2_clean::Config;
use std::fs;

fn assert_ne_empty(s: &str) {
    assert_ne!(0, fs::metadata(temp(s)).unwrap().len());
}

fn run(args: &[&str]) -> Config {
    let mut c = cfg(args);
    let _ = fs::remove_dir_all(&c.output.dir);
    c.output.create_dirs().unwrap();
    c.run().unwrap();
    c
}

fn unzip_to(dir: &str) -> String {
    let i = data("book.fb2.zip").to_str().unwrap().to_owned();
    let o = temp(dir).to_str().unwrap().to_owned();
    run(&["-e", "-i", &i, "-o", &o, "--unzip"]);

    let o = temp(dir).join("book.fb2");
    assert_ne!(0, fs::metadata(&o).unwrap().len());
    o.to_str().unwrap().into()
}

#[test]
fn fb2_file() {
    let i = unzip_to("fb2_file");
    run(&["-e", "-i", &i]);
    assert_ne_empty("fb2_file/cleaned/book.fb2");
}

#[test]
fn fb2_zip_file() {
    let i = data("book.fb2.zip").to_str().unwrap().to_owned();
    let o = temp("fb2_zip_file").to_str().unwrap().to_owned();
    run(&["-e", "-i", &i, "-o", &o]);
    assert_ne_empty("fb2_zip_file/book.fb2.zip");
}

#[test]
fn recursive() {
    let i = data("recursive").to_str().unwrap().to_owned();
    let o = temp("recursive").to_str().unwrap().to_owned();
    run(&["-e", "-i", &i, "-o", &o, "--recursive"]);

    for f in ["dummy.fb2", "1/dummy.fb2", "1/2/3/dummy.fb2"] {
        let f = temp(&format!("recursive/{}", f));
        assert!(f.exists());
    }
}

#[test]
fn recursive_limit() {
    let i = data("recursive").to_str().unwrap().to_owned();
    let o = temp("recursive_limit").to_str().unwrap().to_owned();
    run(&["-e", "-i", &i, "-o", &o, "--recursive", "1"]);

    for f in ["dummy.fb2", "1/dummy.fb2"] {
        let f = temp(&format!("recursive_limit/{}", f));
        assert!(f.exists());
    }
    assert!(!temp("recursive_limit/1/2/3/dummy.fb2").exists());
}

#[test]
fn zip() {
    let i = unzip_to("zip");
    run(&["-e", "-i", &i, "--zip"]);
    assert_ne_empty("zip/cleaned/book.fb2.zip");
}

#[test]
fn unzip() {
    let _ = unzip_to("unzip");
}

#[test]
fn force() {
    let i = unzip_to("force");
    run(&["-ef", "-i", &i, "--zip"]);
    assert_ne_empty("force/book.fb2.zip");
    assert!(!fs::exists(i).unwrap());
}

#[test]
fn force_recursive() {
    let i = data("recursive").to_str().unwrap().to_owned();
    let o = temp("force_recursive").to_str().unwrap().to_owned();
    run(&["-e", "-i", &i, "-o", &o, "--recursive"]);

    let c = run(&["-ef", "-i", &o, "--recursive"]);
    c.output.remove_created_dirs();
    assert!(!fs::exists(c.output.dir).unwrap());
}

#[test]
fn exit_on_err() {
    let i = data("dummy.fb2").to_str().unwrap().to_owned();
    let o = temp("exit_on_err").to_str().unwrap().to_owned();

    let c = cfg(&["-i", &i, "-o", &o]);
    c.run().unwrap();

    let c = cfg(&["-e", "-i", &i, "-o", &o]);
    c.run().unwrap_err();
}
