use std::{
    path::{PathBuf}
};

/// This is the main data structure used to store the configuration
/// of the app.
///
/// # Notes
/// * This must be serializeable because it needs to be stored in a
/// file (obviously)
/// * Some fields are probably better farmed out to other structs to
/// make things more modular and obvious.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApplicationConfig {
    pub dropbox_auth: DropboxAuthConfig,
    pub dropbox_directory: PathBuf,
    pub ignore_folders: Vec<PathBuf>,

}

/// Likely this will wrap a Dropbox SDK provided type, but I
/// needed to blackbox it somehow.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DropboxAuthConfig {

}