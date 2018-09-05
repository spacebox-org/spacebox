use super::{
    Result
};

mod dropbox_backend;
pub use self::dropbox_backend::{
    DropboxBackend,
    DropboxAuthToken,
    DropboxSessionConfig
};

/// A trait to represent the important aspects of a backend to
/// the other layers of Spacebox.  This needs to capture at least
/// all of the important parts of standard file system operations.
/// Some acceptable losses for the sake of generality are things
/// like rename operations that we can do with three CRUD
/// operations.
pub trait StorageBackend {
    /// A backend specific authentication structure that can be used to
    /// authenticate with the remote (?) server.
    type AuthToken;

    /// A backend specific general settings structure that is used to
    /// configure the backend at startup.
    type SessionConfig;

    /// Initialize the backend for a session of communication with the server.
    fn new(token: Self::AuthToken, session_config: Self::SessionConfig) -> Self;

    /// Create a new file on the storage backend.
    fn create_file(&self) -> Result<()>;

    /// Read a file from the storage backend.
    fn read_file(&self) -> Result<()>;

    /// Update a file (overwrite) on the storage backend.
    fn update_file(&self) -> Result<()>;

    /// Delete a file on the storage backend.
    fn delete_file(&self) -> Result<()>;

    /// Check the total space on this storage backend.
    fn get_total_space(&self) -> Result<()>;

    /// Check the remaining space on this storage backend.
    fn get_used_space(&self) -> Result<()>;
}