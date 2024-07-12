use args::Args;
use tracing::{info, Level};
use tracing_subscriber::{
    filter,
    fmt::{self, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer, Registry,
};

mod args;

const LOG_LEVEL_KEY: &str = "LOG_LEVEL";
const LOG_LEVEL_DEFAULT: Level = Level::INFO;

pub struct Config {
    args: Args,
}

//TODO: read yaml input for log path

impl Config {
    pub fn new(args: &Vec<String>) -> Self {
        let config = Config {
            args: Args::new(args),
        };

        config.init_logging();
        config
    }

    fn init_logging(&self) {
        let level = match self.args.get_key(LOG_LEVEL_KEY) {
            Some(l) => Self::to_level(l),
            None => LOG_LEVEL_DEFAULT,
        };

        let file_appender = tracing_appender::rolling::never("./logs", "logfile.log");
        let terminal_layer = fmt::layer()
            .with_writer(std::io::stdout.with_max_level(level))
            .with_ansi(true)
            .pretty();

        let subscriber = Registry::default()
            .with(
                terminal_layer, //     fmt::layer().compact().with_ansi(true).with_filter(filter::LevelFilter::from_level(Level::INFO))
            )
            .with(fmt::layer().with_writer(file_appender));

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
