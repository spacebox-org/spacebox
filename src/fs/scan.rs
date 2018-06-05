use std::path::{Path};
use std::fs;

pub enum FileChange<P: AsRef<Path>> {
    Creation(P),
    Deletion(P),
    Modification(P),
}

/// Scans a given directory recursively for changes in files by comparing them against 
/// a database of known hashes of the files.
///
/// If a file is not found in the database of known files, then it is considered new,
/// and is tagged as `FileChange::Creation(path)`.
///
/// If a file is in the database, but has a different stored hash then it is considered
/// modified and is tagged as `FileChange::Modification(path)`.
///
/// If a path is in the database, but not found at the expected path, it is considered
/// to have been deleted and tagged as `FileChange::Deletion(path)`.
pub fn scan_directory<P: AsRef<Path>>(path: P, cache_file: P) -> Vec<FileChange<P>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    /// Tests the case when there is no metadata file present, in which case, everything
    /// should be considered as a new file.
    fn no_metadata() {

    }

    #[test]
    /// Tests the case when there are no changes to the directory, in which case, the
    /// return `Vec<FileChange<P>>` should be empty.
    fn no_changes() {

    }

    #[test]
    /// Tests when there are a few, unmodified files in a single layer directory and
    /// several more are added.  Should return a `Vec<FileChange<P>>` with only
    /// `FileChange::Creation`
    fn file_additions() {

    }

    #[test]
    fn file_modifications() {

    }

    #[test]
    fn file_deletions() {

    }

    #[test]
    /// Tests a mix of addition, deletion, and modification in a single layer directory
    fn basic_scan() {
        
    }

    

    #[test]
    fn file_moves() {

    }

    #[test]
    fn file_copies() {

    }

    

    

}