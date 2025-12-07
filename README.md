# fb2-clean

A CLI utility for clean fb2 files.

[![Tests](https://github.com/nujievik/fb2-clean/actions/workflows/tests.yml/badge.svg)](
https://github.com/nujievik/fb2-clean/actions/workflows/tests.yml)


## Quick Start

1. [Download](https://github.com/nujievik/fb2-clean/releases) the
archive for your system.

2. Unpack it.

3. Run the unpacked `fb2-clean` in a directory with fb2.


## Default Behaviour

- Saves cleaned files to the `cleaned` subdirectory.

- Cleans all `fb2` and `fb2.zip` in a directory.

- Removes `binary`, `coverpage`, and `image` tags.

- Keeps input extension: saves `fb2` as `fb2` and `fb2.zip` as
`fb2.zip`.


## Advanced Use 🤓

Run `fb2-clean -h` to display help.

| Option                  | Description                       |
|-------------------------|-----------------------------------|
| `-i, --input <path>`    | Input directory OR file           |
| `-o, --output <dir>`    | Output directory                  |
| `-t, --tags <n[,m...]>` | Remove tags                       |
| `-z, --zip`             | Compress fb2 to fb2.zip           |
| `-Z, --unzip`           | Uncompress fb2.zip to fb2         |
| `-f, --force`           | Force overwrite existing files    |
| `-e, --exit-on-err`     | Skip clean next files on error    |


## Manual Build 🤓

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Clone the repo:
```
git clone https://github.com/nujievik/fb2-clean --depth 1
```

3. Enter the project directory:
```
cd fb2-clean
```

4. Build:
```
cargo build --release
```

5. On success, the binary will be in `target/release/fb2-clean`
