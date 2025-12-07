use clap::Parser;
use fb2_clean::{Config, Result};

fn main() -> Result<()> {
    let mut cfg = Config::parse();
    cfg.output.create_dirs()?;

    let res = cfg.run();
    cfg.output.remove_created_dirs();
    res
}
