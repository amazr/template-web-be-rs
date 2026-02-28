use std::{future::Future, pin::Pin, time::Duration};

use tracing::info;

use crate::tasks::{SchedulableTask, TaskContext};

#[derive(Debug, Clone)]
pub struct HeartbeatTask {
    interval: Duration,
}

impl HeartbeatTask {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }
}

impl SchedulableTask for HeartbeatTask {
    fn name(&self) -> &'static str {
        "heartbeat"
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn run_once(&self, ctx: TaskContext) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if ctx.cancel_token().is_cancelled() {
                return;
            }

            info!("heartbeat tick");
        })
    }
}
