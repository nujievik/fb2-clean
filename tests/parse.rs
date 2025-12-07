#[allow(unused)]
mod common;

use common::*;
use fb2_clean::*;
use std::{collections::HashSet, fs, path::Path, sync::LazyLock};

fn output_from_i(dir: &Path) -> Output {
    Output {
        dir: dir.join("cleaned").into(),
        created_dirs: Vec::new(),
    }
}

#[test]
fn empty_args() {
    let idir = fs::canonicalize(".").unwrap().into_boxed_path();
    let exp = Config {
        output: output_from_i(&idir),
        input: Input::Dir(idir),
        tags: Default::default(),
        zip: false,
        unzip: false,
        force: false,
        exit_on_err: false,
    };

    assert_eq!(exp, cfg(&[]));
}

fn eq_empty_without_io(c: &Config) {
    static EMPTY: LazyLock<Config> = LazyLock::new(|| cfg(&[]));

    assert_eq!(&c.tags, &EMPTY.tags);
    assert_eq!(c.zip, EMPTY.zip);
    assert_eq!(c.unzip, EMPTY.unzip);
    assert_eq!(c.force, EMPTY.force);
    assert_eq!(c.exit_on_err, EMPTY.exit_on_err);
}

#[test]
fn input_dir() {
    for dir in ["", "tests"] {
        let idir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let idir = fs::canonicalize(idir).unwrap().into_boxed_path();
        let c = cfg(&["--input", idir.to_str().unwrap()]);

        assert_eq!(c.output, output_from_i(&idir));
        assert_eq!(c.input, Input::Dir(idir));
        eq_empty_without_io(&c);
    }
}

#[test]
fn input_file() {
    for &(ty, f) in ITERABLE.iter() {
        let i = fs::canonicalize(data(f)).unwrap().into_boxed_path();
        let c = cfg(&["--input", i.to_str().unwrap()]);

        assert_eq!(c.output.dir, i.parent().unwrap().join("cleaned").into());
        assert_eq!(c.output.created_dirs, Vec::new());
        assert_eq!(c.input, Input::File(InputFile { ty, path: i }));
        eq_empty_without_io(&c);
    }
}

#[test]
fn output() {
    for dir in ["", "tests"] {
        let odir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let odir = fs::canonicalize(odir).unwrap().into_boxed_path();
        let c = cfg(&["--output", odir.to_str().unwrap()]);

        assert_eq!(c.input, Input::Dir(fs::canonicalize(".").unwrap().into()));
        assert_eq!(c.output.dir, odir);
        assert_eq!(c.output.created_dirs, Vec::new());
        eq_empty_without_io(&c);
    }
}

#[test]
fn tags() {
    for tags in ["a", "b", "a,b", "a,c"] {
        let exp: HashSet<Box<[u8]>> = tags.split(',').map(|t| t.as_bytes().into()).collect();
        let mut c = cfg(&["--tags", tags]);

        assert_eq!(c.tags, Tags(exp));
        c.tags = Default::default();
        assert_eq!(c, cfg(&[]));
    }
}

#[test]
fn zip() {
    let mut c = cfg(&["--zip"]);
    assert!(c.zip);
    c.zip = false;
    assert_eq!(c, cfg(&[]));
}

#[test]
fn unzip() {
    let mut c = cfg(&["--unzip"]);
    assert!(c.unzip);
    c.unzip = false;
    assert_eq!(c, cfg(&[]));
}

#[test]
fn force() {
    let mut c = cfg(&["--force"]);
    assert!(c.force);
    c.force = false;
    assert_eq!(c, cfg(&[]));
}

#[test]
fn exit_on_err() {
    let mut c = cfg(&["--exit-on-err"]);
    assert!(c.exit_on_err);
    c.exit_on_err = false;
    assert_eq!(c, cfg(&[]));
}

#[test]
fn aliases_io_tags() {
    let v = env!("CARGO_MANIFEST_DIR");

    for xs in [["-i", "--input"], ["-o", "--output"], ["-t", "--tags"]] {
        let first = cfg(&[xs[0], v]);

        for x in &xs[1..] {
            assert_eq!(&first, &cfg(&[x, v]));
        }
    }
}

#[test]
fn aliases() {
    [
        vec!["-z", "--zip"],
        vec!["-Z", "--unzip", "--no-zip"],
        vec!["-f", "--force"],
        vec!["-e", "--exit-on-err", "--exit-on-error"],
    ]
    .iter()
    .for_each(|xs| {
        let first = cfg(&[xs[0]]);

        for x in &xs[1..] {
            assert_eq!(&first, &cfg(&[x]));
        }
    })
}
