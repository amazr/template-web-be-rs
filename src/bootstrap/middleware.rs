use axum::{
    Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use opentelemetry::trace::TraceContextExt;
use tracing::error;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::errors::ErrorResponse;

pub async fn trace_id_header(req: Request<Body>, next: Next) -> Response {
    let mut response = next.run(req).await;
    let trace_id = tracing::Span::current()
        .context()
        .span()
        .span_context()
        .trace_id()
        .to_string();

    response
        .headers_mut()
        .insert("x-trace-id", trace_id.parse().unwrap());
    response
}

pub fn panic_handler(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "unknown panic".to_string()
    };

    error!(message = %message, "service panicked");
    let body = Json(ErrorResponse::InternalServerError { error: message });
    (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
}
