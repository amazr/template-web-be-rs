use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    store::user_store::UserStore,
    tasks::{BackgroundTask, spawn_instrumented_task},
};

type TaskFactory = Arc<dyn Fn() -> Box<dyn BackgroundTask> + Send + Sync>;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    task_factories: Vec<TaskFactory>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            task_factories: Vec::new(),
        }
    }

    pub fn user_store(&self) -> UserStore {
        UserStore::new(self.db.clone())
    }

    pub fn register_task<T>(&mut self, task: T)
    where
        T: BackgroundTask + Clone + Send + Sync + 'static,
    {
        self.register_task_factory(move || Box::new(task.clone()));
    }

    pub fn register_task_factory<F>(&mut self, factory: F)
    where
        F: Fn() -> Box<dyn BackgroundTask> + Send + Sync + 'static,
    {
        self.task_factories.push(Arc::new(factory));
    }

    pub async fn start_tasks(self, cancel_token: CancellationToken) {
        let mut tasks = tokio::task::JoinSet::<()>::new();

        for factory in self.task_factories {
            spawn_instrumented_task(&mut tasks, factory(), cancel_token.clone());
        }

        if tasks.is_empty() {
            info!("No background tasks scheduled");
            return;
        }

        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                error!("Task failed: {e:?}");
            }
        }

        info!("All background tasks shut down");
    }
}
