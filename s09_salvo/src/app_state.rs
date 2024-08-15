use std::sync::Arc;
use crate::storage::storage::Storage;

#[derive(Debug, Clone)]
pub struct AppState {
    pub storage: Arc<Storage>
}