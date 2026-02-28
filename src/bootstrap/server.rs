use std::future::Future;

use axum::Router;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn run<T>(
    address: String,
    port: u16,
    app: Router,
    cancel_token: CancellationToken,
    tasks: T,
) where
    T: Future<Output = ()>,
{
    let addr = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind tcp listener");
    let server =
        axum::serve(listener, app).with_graceful_shutdown(shutdown_hook(cancel_token.clone()));

    info!("Server listening on http://{addr}");
    let (_, server_result) = tokio::join!(tasks, server);
    server_result.expect("server failed");
}

async fn shutdown_hook(cancel_token: CancellationToken) {
    tokio::signal::ctrl_c().await.ok();
    info!("Shutting down");
    cancel_token.cancel();
}
