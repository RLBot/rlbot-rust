use std::path::{Path, PathBuf};

pub fn maybe_join(base: Option<&Path>, path: impl AsRef<Path>) -> PathBuf {
    match base {
        Some(base) => base.join(path),
        None => path.as_ref().to_path_buf(),
    }
}
