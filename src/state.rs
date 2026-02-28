use sea_orm::DatabaseConnection;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::store::user_store::UserStore;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn user_store(&self) -> UserStore {
        UserStore::new(self.db.clone())
    }

    pub async fn start_tasks(self, _cancel_token: CancellationToken) {
        let mut tasks = tokio::task::JoinSet::<()>::new();

        if !tasks.is_empty() {
            while let Some(res) = tasks.join_next().await {
                if let Err(e) = res {
                    error!("Task failed: {e:?}");
                }
            }
            info!("All background tasks shut down");
        } else {
            info!("No background tasks scheduled");
        }
    }
}
