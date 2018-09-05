use super::{
    Result,
    StorageBackend
};

/// A struct representing a backend to store things on Dropbox.
pub struct DropboxBackend {

}

/// A Dropbox specific authentication token.
pub struct DropboxAuthToken {

}

/// A Dropbox specific session config.
pub struct DropboxSessionConfig {

}

#[allow(unused)]
impl StorageBackend for DropboxBackend {
    type AuthToken = DropboxAuthToken;
    type SessionConfig = DropboxSessionConfig;

    fn new(token: Self::AuthToken, session_config: Self::SessionConfig) -> Self {
        DropboxBackend {}
    }

    fn create_file(&self) -> Result<()> {
        unimplemented!()
    }

    fn read_file(&self) -> Result<()> {
        unimplemented!()
    }

    fn update_file(&self) -> Result<()> {
        unimplemented!()
    }

    fn delete_file(&self) -> Result<()> {
        unimplemented!()
    }

    fn get_total_space(&self) -> Result<()> {
        unimplemented!()
    }

    fn get_used_space(&self) -> Result<()> {
        unimplemented!()
    }
}