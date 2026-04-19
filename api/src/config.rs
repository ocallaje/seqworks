use std::env;
use std::path::PathBuf;


pub fn data_root() -> Result<PathBuf, String> {
    env::var("DATA_ROOT")
        .map(PathBuf::from)
        .map_err(|_| "DATA_ROOT not set".to_string())
}

