use serde::Deserialize;

const DEFAULT_HOST: &str = "10.0.5.255";
const DEFAULT_LOG_PATH: &str = "./logs";
const DEFAULT_LOG_FILE: &str = "smash.log";

#[derive(Deserialize, PartialEq, Debug)]
pub struct ConfigFile {
    hosts: Vec<String>,
    log_path: String,
    log_file: String,
}

impl ConfigFile {
    pub fn new(content: &str) -> Self {
        if let Ok(config) = serde_yml::from_str(content) {
            config
        } else {
            eprintln!("Failed to parsed config, using default configuration!");
            Self {
                hosts: vec![DEFAULT_HOST.to_owned()],
                log_path: DEFAULT_LOG_PATH.to_owned(),
                log_file: DEFAULT_LOG_FILE.to_owned(),
            }
        }
    }

    pub fn get_hosts(&self) -> &Vec<String> {
        &self.hosts
    }

    pub fn get_log_path(&self) -> &str {
        self.log_path.as_str()
    }
    pub fn get_log_file(&self) -> &str {
        self.log_file.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // no content
        let content = "";

        let result = ConfigFile::new(content);

        let expected = ConfigFile {
            hosts: vec![DEFAULT_HOST.to_owned()],
            log_path: DEFAULT_LOG_PATH.to_owned(),
            log_file: DEFAULT_LOG_FILE.to_owned(),
        };

        assert_eq!(result, expected);

        // wrong content
        let content = r#"
hosts:
  - "10.0.5.255"
"#;

        let result = ConfigFile::new(content);

        let expected = ConfigFile {
            hosts: vec![DEFAULT_HOST.to_owned()],
            log_path: DEFAULT_LOG_PATH.to_owned(),
            log_file: DEFAULT_LOG_FILE.to_owned(),
        };

        assert_eq!(result, expected);

        // correct content
        let content = r#"
hosts:
  - "10.0.5.23"
  - "10.0.5.42"
log_path: "./logs"
log_file: "log"
"#;

        let result = ConfigFile::new(content);

        let expected = ConfigFile {
            hosts: vec!["10.0.5.23".to_owned(), "10.0.5.42".to_owned()],
            log_path: "./logs".to_owned(),
            log_file: "log".to_owned(),
        };

        assert_eq!(result, expected);
    }
}
