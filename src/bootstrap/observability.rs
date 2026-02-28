use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::info;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(rust_log: &str) {
    let provider = SdkTracerProvider::builder().build();
    let tracer = provider.tracer(env!("CARGO_PKG_NAME"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::new(rust_log))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    info!("Log level set to: {rust_log}");
}
