use super::{
    Result
};

pub trait StorageBackend {
    type AuthToken;
    type SessionConfig;
    fn new(token: Self::AuthToken, session_config: Self::SessionConfig) -> Self;
    fn create_file(&self) -> Result<()>;
    fn read_file(&self) -> Result<()>;
    fn update_file(&self) -> Result<()>;
    fn delete_file(&self) -> Result<()>;
    fn get_total_space(&self) -> Result<()>;
    fn get_used_space(&self) -> Result<()>;
}