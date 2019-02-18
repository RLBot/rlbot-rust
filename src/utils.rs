use std::path::{Path, PathBuf};

pub fn maybe_join(base: Option<&Path>, path: impl AsRef<Path>) -> PathBuf {
    match base {
        Some(base) => base.join(path),
        None => path.as_ref().to_path_buf(),
    }
}

/// Iterates over a `flatbuffers::Vector`.
pub fn flat_vector_iter<'a, T: flatbuffers::Follow<'a>>(
    xs: flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<T>>,
) -> impl Iterator<Item = <T as flatbuffers::Follow<'_>>::Inner> {
    (0..xs.len()).map(move |i| xs.get(i))
}
