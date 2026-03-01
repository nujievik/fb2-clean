#[cfg(feature = "cli")]
fn main() -> fb2_clean::Result<()> {
    use clap::Parser;
    let mut cfg = fb2_clean::Config::parse();
    cfg.output.create_dirs()?;

    fb2_clean::cli::CliLogger::init();
    let res = cfg.run();
    cfg.output.remove_created_dirs();
    res
}
