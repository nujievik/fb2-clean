use super::{Config, Input, Output, Tags};
use clap::{
    Arg, ArgAction, ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser,
    builder::{TypedValueParser, ValueParser},
    error::{ContextKind, ContextValue, ErrorKind},
    value_parser,
};
use std::ffi::OsStr;

type Result<T> = std::result::Result<T, clap::Error>;

impl Parser for Config {}

impl FromArgMatches for Config {
    fn from_arg_matches(m: &ArgMatches) -> Result<Config> {
        Self::from_arg_matches_mut(&mut m.clone())
    }

    fn update_from_arg_matches(&mut self, m: &ArgMatches) -> Result<()> {
        self.update_from_arg_matches_mut(&mut m.clone())
    }

    fn from_arg_matches_mut(m: &mut ArgMatches) -> Result<Config> {
        let input = match m.remove_one::<Input>("input") {
            Some(i) => i,
            None => Input::new(".").unwrap(),
        };
        let output = match m.remove_one::<Output>("output") {
            Some(o) => o,
            None => Output::try_from_input(&input).unwrap(),
        };

        Ok(Config {
            input,
            output,
            recursive: *m.get_one::<u8>("recursive").unwrap_or(&0),
            tags: m
                .remove_one::<Tags>("tags")
                .unwrap_or_else(|| Tags::default()),
            zip: m.get_flag("zip"),
            unzip: m.get_flag("unzip"),
            force: m.get_flag("force"),
            exit_on_err: m.get_flag("exit-on-err"),
        })
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<()> {
        *self = Self::from_arg_matches_mut(m)?;
        Ok(())
    }
}

impl CommandFactory for Config {
    fn command() -> Command {
        Command::new(env!("CARGO_PKG_NAME"))
            .version(concat!("v", env!("CARGO_PKG_VERSION")))
            .disable_help_flag(true)
            .disable_version_flag(true)
            .override_usage(concat!(env!("CARGO_PKG_NAME"), " [options]"))
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("path")
                    .help("Input directory OR file")
                    .value_parser(ValueParser::new(InputParser)),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("dir")
                    .help("Output directory")
                    .value_parser(ValueParser::new(OutputParser)),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .value_name("n")
                    .help("Recursive file search [up to n]")
                    .num_args(0..=1)
                    .default_missing_value("16")
                    .value_parser(value_parser!(u8).range(1..)),
            )
            .arg(
                Arg::new("tags")
                    .short('t')
                    .long("tags")
                    .value_name("n[,m...]")
                    .help("Remove tags")
                    .value_parser(ValueParser::new(TagsParser)),
            )
            .arg(
                Arg::new("zip")
                    .short('z')
                    .long("zip")
                    .help("Compress fb2 to fb2.zip")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("unzip")
                    .short('Z')
                    .long("unzip")
                    .alias("no-zip")
                    .help("Uncompress fb2.zip to fb2")
                    .conflicts_with("zip")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Force overwrite input files")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("exit-on-err")
                    .short('e')
                    .long("exit-on-err")
                    .alias("exit-on-error")
                    .help("Skip clean next files on error")
                    .action(ArgAction::SetTrue),
            )
            .next_help_heading("Other")
            .arg(
                Arg::new("version")
                    .short('V')
                    .long("version")
                    .help("Show version")
                    .action(ArgAction::Version),
            )
            .arg(
                Arg::new("help")
                    .short('h')
                    .long("help")
                    .help("Show help")
                    .action(ArgAction::Help),
            )
    }

    fn command_for_update() -> Command {
        Self::command()
    }
}

macro_rules! ty_parser {
    ($parser:ident, $ty:ty, $try_from_os_str:path) => {
        #[derive(Clone)]
        pub(super) struct $parser;

        impl TypedValueParser for $parser {
            type Value = $ty;

            fn parse_ref(
                &self,
                cmd: &Command,
                arg: Option<&Arg>,
                value: &OsStr,
            ) -> Result<Self::Value> {
                $try_from_os_str(value).map_err(|e| {
                    let mut err = Error::new(ErrorKind::InvalidValue).with_cmd(&cmd);

                    if let Some(arg) = arg {
                        err.insert(
                            ContextKind::InvalidArg,
                            ContextValue::String(arg.to_string()),
                        );
                    }
                    err.insert(
                        ContextKind::InvalidValue,
                        ContextValue::String(format!(
                            "'{}' (reason: {})",
                            value.to_string_lossy(),
                            e
                        )),
                    );

                    err
                })
            }
        }
    };
}

ty_parser!(InputParser, Input, Input::new);
ty_parser!(OutputParser, Output, Output::new);
ty_parser!(TagsParser, Tags, Tags::fallible_new);
