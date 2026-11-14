use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn internal_config_load(path: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut flags = HashMap::new();

    for (line_index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(invalid_config_line(line_index + 1, "expected `key=value`"));
        };

        let key = key.trim();
        if key.is_empty() {
            return Err(invalid_config_line(line_index + 1, "key cannot be empty"));
        }

        flags.insert(key.to_string(), value.trim().to_string());
    }

    Ok(flags)
}

fn invalid_config_line(line: usize, reason: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid config at line {line}: {reason}"),
    )
}
