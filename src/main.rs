use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::{
    bootstrap::{app, database, observability, server},
    config::AppConfig,
    state::AppState,
    tasks::heartbeat::HeartbeatTask,
};

mod api;
mod bootstrap;
mod config;
mod errors;
mod state;
mod store;
mod tasks;

#[allow(unused)]
mod entities;

#[tokio::main]
async fn main() {
    let config = AppConfig::load();
    observability::init_tracing(&config.rust_log);

    let db = database::connect_and_migrate(&config.db_url).await;

    let mut state = AppState::new(db);
    state.register_task(HeartbeatTask::new(Duration::from_secs(30)));

    let cancel_token = CancellationToken::new();
    let tasks = state.clone().start_tasks(cancel_token.clone());
    let app = app::build_router(state);

    server::run(config.address, config.port, app, cancel_token, tasks).await;
}
