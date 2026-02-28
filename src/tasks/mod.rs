use std::{future::Future, pin::Pin, time::Duration};

use opentelemetry::trace::TraceContextExt;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub mod heartbeat;

#[derive(Clone, Debug)]
pub struct TaskContext {
    cancel_token: CancellationToken,
}

impl TaskContext {
    pub fn new(cancel_token: CancellationToken) -> Self {
        Self { cancel_token }
    }

    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel_token.clone()
    }
}

pub trait SchedulableTask: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn interval(&self) -> Duration;
    fn run_once(&self, ctx: TaskContext) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub trait BackgroundTask: Send + 'static {
    fn name(&self) -> &'static str;
    fn run(self: Box<Self>, cancel_token: CancellationToken) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<T> BackgroundTask for T
where
    T: SchedulableTask,
{
    fn name(&self) -> &'static str {
        SchedulableTask::name(self)
    }

    fn run(self: Box<Self>, cancel_token: CancellationToken) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let task = self;
        let interval = task.interval();

        Box::pin(async move {
            let mut ticker = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        tracing::info!("{} task shutting down", task.name());
                        break;
                    }
                    _ = ticker.tick() => {
                        task.run_once(TaskContext::new(cancel_token.clone())).await;
                    }
                }
            }
        })
    }
}

pub fn spawn_instrumented_task(
    tasks: &mut JoinSet<()>,
    task: Box<dyn BackgroundTask>,
    cancel_token: CancellationToken,
) {
    let task_name = task.name();
    let span = info_span!(
        "background_task",
        task_name = task_name,
        trace_id = tracing::field::Empty,
    );
    record_trace_id(&span);

    tasks.spawn(task.run(cancel_token).instrument(span));
}

fn record_trace_id(span: &tracing::Span) {
    let trace_id = span.context().span().span_context().trace_id().to_string();
    span.record("trace_id", trace_id);
}
