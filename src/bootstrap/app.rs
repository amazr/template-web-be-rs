use axum::{middleware, Router};
use opentelemetry::trace::TraceContextExt;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
use tracing::info_span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::users::users_endpoints,
    bootstrap::middleware::{panic_handler, trace_id_header},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    let (app, openapi) = OpenApiRouter::<AppState>::new()
        .nest(
            "/api",
            OpenApiRouter::new().nest("/users", users_endpoints::router()),
        )
        .split_for_parts();

    app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .layer(CatchPanicLayer::custom(panic_handler))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(middleware::from_fn(trace_id_header))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        trace_id = tracing::field::Empty,
                    )
                })
                .on_request(|_req: &axum::http::Request<_>, span: &tracing::Span| {
                    let trace_id = span.context().span().span_context().trace_id().to_string();
                    span.record("trace_id", trace_id);
                }),
        )
        .with_state(state)
}
