#[allow(unused)]
mod common;

use common::*;
use std::{fs, path::Path};

fn unzip_to(dir: &str) -> Box<Path> {
    let i = data("book.fb2.zip").to_str().unwrap().to_owned();
    let o = temp(dir).to_str().unwrap().to_owned();

    let mut c = cfg(&["-ef", "-i", &i, "-o", &o, "--unzip"]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    temp(dir).join("book.fb2").into()
}

#[test]
fn fb2_file() {
    let i = unzip_to("fb2_file");
    let mut c = cfg(&["-ef", "-i", i.to_str().unwrap()]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    let o = i.parent().unwrap().join("cleaned/book.fb2");
    assert_ne!(0, fs::metadata(&o).unwrap().len());
}

#[test]
fn fb2_zip_file() {
    let i = data("book.fb2.zip").to_str().unwrap().to_owned();
    let o = temp("fb2_zip_file").to_str().unwrap().to_owned();

    let mut c = cfg(&["-ef", "-i", &i, "-o", &o]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    let o = temp("fb2_zip_file").join("book.fb2.zip");
    assert_ne!(0, fs::metadata(&o).unwrap().len());
}

#[test]
fn recursive() {
    let i = data("recursive").to_str().unwrap().to_owned();
    let o = temp("recursive").to_str().unwrap().to_owned();
    let _ = fs::remove_dir_all(&o);

    let mut c = cfg(&["-ef", "-i", &i, "-o", &o, "--recursive"]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    for f in ["dummy.fb2", "1/dummy.fb2", "1/2/3/dummy.fb2"] {
        let f = temp(&format!("recursive/{}", f));
        assert!(f.exists());
    }
}

#[test]
fn recursive_limit() {
    let i = data("recursive").to_str().unwrap().to_owned();
    let o = temp("recursive_limit").to_str().unwrap().to_owned();
    let _ = fs::remove_dir_all(&o);

    let mut c = cfg(&["-ef", "-i", &i, "-o", &o, "--recursive", "1"]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    for f in ["dummy.fb2", "1/dummy.fb2"] {
        let f = temp(&format!("recursive_limit/{}", f));
        assert!(f.exists());
    }
    assert!(!temp("recursive_limit/1/2/3/dummy.fb2").exists());
}

#[test]
fn zip() {
    let i = unzip_to("zip");
    assert_ne!(0, fs::metadata(&i).unwrap().len());

    let mut c = cfg(&["-ef", "-i", i.to_str().unwrap(), "--zip"]);
    c.output.create_dirs().unwrap();
    c.run().unwrap();

    let o = i.parent().unwrap().join("cleaned/book.fb2.zip");
    assert_ne!(0, fs::metadata(&o).unwrap().len());
}

#[test]
fn unzip() {
    let o_file = unzip_to("unzip");
    assert_ne!(0, fs::metadata(&o_file).unwrap().len());
}

#[test]
fn force() {
    let i = data("book.fb2.zip").to_str().unwrap().to_owned();
    let o = temp("force").to_str().unwrap().to_owned();
    let o_file = temp("force/book.fb2.zip");

    let mut c = cfg(&["-e", "-i", &i, "-o", &o]);
    c.output.create_dirs().unwrap();
    let _ = fs::File::create(&o_file);

    c.run().unwrap();
    assert_eq!(0, fs::metadata(&o_file).unwrap().len());
    c.force = true;
    c.run().unwrap();
    assert_ne!(0, fs::metadata(&o_file).unwrap().len());
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
