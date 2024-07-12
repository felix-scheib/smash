use std::collections::HashMap;

const DELIMITER: char = '=';

pub struct Args {
    args: HashMap<String, String>,
}

impl Args {
    pub fn new(args: &Vec<String>) -> Self {
        Args {
            args: extract_args(args),
        }
    }

    pub fn get_key(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }
}

fn extract_args(args: &Vec<String>) -> HashMap<String, String> {
    args.iter()
        .filter_map(|x| {
            x.split_once(DELIMITER)
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .filter(|(k, v)| k.len() > 0 && v.len() > 0)
        })
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_args() {
        // empty Vec
        let args = Vec::new();

        let result = extract_args(&args);
        let expected = HashMap::new();

        assert_eq!(result, expected);

        // wrong format: "":""
        let args = vec!["=".to_owned()];

        let result = extract_args(&args);
        let expected = HashMap::new();

        assert_eq!(result, expected);

        // wrong format: "foo":""
        let args = vec!["foo=".to_owned()];

        let result = extract_args(&args);
        let expected = HashMap::new();

        assert_eq!(result, expected);

        // wrong format: "":"foo"
        let args = vec!["=foo".to_owned()];

        let result = extract_args(&args);
        let expected = HashMap::new();

        assert_eq!(result, expected);

        // right format
        let (k1, v1) = ("foo", "bar");
        let (k2, v2) = ("bar", "foo");

        let args = vec![format!("{k1}={v1}"), format!("{k2}={v2}")];

        let result = extract_args(&args);
        let expected = HashMap::from([
            (k1.to_owned(), v1.to_owned()),
            (k2.to_owned(), v2.to_owned()),
        ]);

        assert_eq!(result, expected);

        // some right, some wrong
        let (k1, v1) = ("foo", "bar");
        let (k2, v2) = ("bar", "foo");

        let args = vec![format!("{k1}={v1}"), format!("{k2}{v2}")];

        let result = extract_args(&args);
        let expected = HashMap::from([(k1.to_owned(), v1.to_owned())]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_key() {
        // key found
        let (k1, v1) = ("foo", "bar");
        let (k2, v2) = ("bar", "foo");

        let mock = Args {
            args: HashMap::from([
                (k1.to_owned(), v1.to_owned()),
                (k2.to_owned(), v2.to_owned()),
            ]),
        };

        let result = mock.get_key(k1);
        let val = v1.to_owned();
        let expected = Some(&val);

        assert_eq!(result, expected);

        // key not found
        let result = mock.get_key("bizz");
        let expected = None;

        assert_eq!(result, expected);
    }
}
