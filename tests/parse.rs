#[allow(unused)]
mod common;

use clap::Parser;
use common::*;
use fb2_clean::*;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

fn output_from_i(dir: &Path) -> Output {
    Output {
        dir: dir.join("cleaned").into(),
        len_created_dir_chain: 0,
    }
}

#[test]
fn empty_args() {
    let c = cfg(&[]);
    let idir = fs::canonicalize(".").unwrap().into_boxed_path();
    let o = output_from_i(&idir);

    assert_eq!(c.input, Input::Dir(idir));
    assert_eq!(c.output, o);
    assert_eq!(c.recursive, 16);
    assert_eq!(c.tags, Tags::default());
    assert!(!c.zip);
    assert!(!c.unzip);
    assert!(!c.overwrite);
    assert!(!c.exit_on_err);
    assert_eq!(c.jobs, rayon::current_num_threads() as u64);
}

fn eq_empty_without_io(c: &Config) {
    static EMPTY: LazyLock<Config> = LazyLock::new(|| cfg(&[]));

    assert_eq!(&c.tags, &EMPTY.tags);
    assert_eq!(c.zip, EMPTY.zip);
    assert_eq!(c.unzip, EMPTY.unzip);
    assert_eq!(c.overwrite, EMPTY.overwrite);
    assert_eq!(c.exit_on_err, EMPTY.exit_on_err);
    assert_eq!(c.jobs, EMPTY.jobs);
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
        assert_eq!(c.output.len_created_dir_chain, 0);
        assert_eq!(c.input, Input::Files(vec![InputFile { ty, path: i }]));
        eq_empty_without_io(&c);
    }
}

#[test]
fn input_files() {
    let mut args: Vec<String> = Vec::new();
    let mut i_files: Vec<InputFile> = Vec::new();
    let mut o_dir: Option<PathBuf> = None;

    for &(ty, f) in ITERABLE.iter() {
        let i = fs::canonicalize(data(f)).unwrap().into_boxed_path();

        let _ = o_dir.get_or_insert(i.parent().unwrap().join("cleaned").into());
        args.push("-i".into());
        args.push(data(f).to_str().unwrap().into());
        i_files.push(InputFile { ty, path: i });
    }

    let args_references: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let c = cfg(&args_references);

    assert_eq!(c.output.dir, o_dir.unwrap().into());
    assert_eq!(c.output.len_created_dir_chain, 0);
    assert_eq!(c.input, Input::Files(i_files));
    eq_empty_without_io(&c);
}

#[test]
fn output() {
    for dir in ["", "tests"] {
        let odir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let odir = fs::canonicalize(odir).unwrap().into_boxed_path();
        let c = cfg(&["--output", odir.to_str().unwrap()]);

        assert_eq!(c.input, Input::Dir(fs::canonicalize(".").unwrap().into()));
        assert_eq!(c.output.dir, odir);
        assert_eq!(c.output.len_created_dir_chain, 0);
        eq_empty_without_io(&c);
    }
}

#[test]
fn recursive() {
    for n in [0, 1, 2, 8] {
        let mut c = cfg(&["--recursive", &n.to_string()]);
        assert_eq!(n, c.recursive);
        c.recursive = 16;
        eq_empty_without_io(&c);
    }
}

#[test]
fn tags() {
    use indexmap::IndexSet;
    for tags in ["a", "b", "a,b", "a,c"] {
        let exp: IndexSet<Box<[u8]>> = tags.split(',').map(|t| t.as_bytes().into()).collect();
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
fn zip_unzip_conflict() {
    Config::try_parse_from(&["x", "--zip", "--unzip"]).unwrap_err();
}

#[test]
fn overwrite() {
    let mut c = cfg(&["--overwrite"]);
    assert!(c.overwrite);
    c.overwrite = false;
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
fn flag_aliases() {
    [
        vec!["-z", "--zip"],
        vec!["-Z", "--unzip", "--no-zip"],
        vec!["-w", "--overwrite"],
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

#[test]
fn recursive_alias() {
    assert_eq!(cfg(&["--recursive", "16"]), cfg(&["-r", "16"]));
}
