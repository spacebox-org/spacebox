use super::{
    Result
};

pub struct DropboxBackend {

}

pub struct DropboxAuthToken {

}

pub struct DropboxSessionConfig {

}

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
}