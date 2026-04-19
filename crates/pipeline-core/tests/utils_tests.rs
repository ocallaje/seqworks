use pipeline_core::utils;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

   #[test]
    fn test_get_dirs_bulk() {
        let dir = tempdir().unwrap();

        let data = dir.path().join("data");
        std::fs::create_dir_all(&data).unwrap();
        std::fs::create_dir(data.join("proj1")).unwrap();
        std::fs::create_dir(data.join("proj2")).unwrap();

        let data_root = PathBuf::from(dir.path());

        let dirs = utils::get_dirs(&data_root, "bulk").unwrap();
        assert_eq!(dirs.len(), 2);
    }
}
