use std::{
    fs,
    io::{self, ErrorKind},
};

use args::Args;
use config_file::ConfigFile;
use tracing::{info, Level};
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    layer::SubscriberExt,
    Registry,
};

mod args;
mod config_file;

const LOG_LEVEL_KEY: &str = "LOG";
const LOG_LEVEL_DEFAULT: Level = Level::INFO;
const CONFIG_FILE_KEY: &str = "CONFIG";

pub struct Config {
    args: Args,
    config_file: ConfigFile,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Self {
        let args = Args::new(args);

        println!("Args: {:#?}", args);

        let config = match args.get_key(CONFIG_FILE_KEY) {
            Some(config_file) => fs::read_to_string(config_file),
            None => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "No config file specified",
            )),
        };

        let content = match config {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error while reading config file: {}", e);
                "".to_owned()
            }
        };

        let config = Config {
            args,
            config_file: ConfigFile::new(content.as_str()),
        };

        config.init_logging();
        config
    }

    fn init_logging(&self) {
        let level = match self.args.get_key(LOG_LEVEL_KEY) {
            Some(l) => Self::to_level(l),
            None => LOG_LEVEL_DEFAULT,
        };

        let file_appender = tracing_appender::rolling::never(
            self.config_file.get_log_path(),
            self.config_file.get_log_file(),
        );

        let terminal_layer = fmt::layer()
            .with_writer(std::io::stdout.with_max_level(level))
            .with_ansi(true)
            .with_file(false)
            .with_line_number(false);

        let subscriber = Registry::default().with(terminal_layer).with(
            fmt::layer()
                .with_writer(file_appender)
                .with_file(true)
                .with_line_number(true),
        );

        tracing::subscriber::set_global_default(subscriber).unwrap();

        info!("Tracing initialized with level: {}", level);
    }

    fn to_level(level: &str) -> Level {
        match level.trim().to_uppercase().as_str() {
            "DEBUG" => Level::DEBUG,
            "ERROR" => Level::ERROR,
            "INFO" => Level::INFO,
            "TRACE" => Level::TRACE,
            "WARN" => Level::WARN,
            _ => LOG_LEVEL_DEFAULT,
        }
    }

    pub fn get_hosts(&self) -> &Vec<String> {
        &self.config_file.get_hosts()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_args() {
        // valid values
        let values = vec![
            ("debug", Level::DEBUG),
            ("DEBUG", Level::DEBUG),
            ("error", Level::ERROR),
            ("ERROR", Level::ERROR),
            ("info", Level::INFO),
            ("INFO", Level::INFO),
            ("trace", Level::TRACE),
            ("TRACE", Level::TRACE),
            ("warn", Level::WARN),
            ("WARN", Level::WARN),
            ("   debug ", Level::DEBUG),
            (" error    ", Level::ERROR),
            (" info", Level::INFO),
            ("trace ", Level::TRACE),
            ("warn    ", Level::WARN),
        ];

        for (v, e) in values {
            assert_eq!(Config::to_level(v), e);
        }

        // invalid values
        let values = vec!["debu", "", "foo"];

        for v in values {
            assert_eq!(Config::to_level(v), LOG_LEVEL_DEFAULT);
        }
    }
}
