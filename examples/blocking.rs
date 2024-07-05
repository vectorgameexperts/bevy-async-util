use async_std::task::sleep;
use bevy::{app::PanicHandlerPlugin, log::LogPlugin, prelude::*};
use bevy_async_task::{AsyncTask, AsyncTaskRunner};
use std::time::Duration;

/// You can block with a task runner
fn system1(mut task_executor: AsyncTaskRunner<u32>) {
    let result = task_executor.blocking_recv(async {
        sleep(Duration::from_millis(1000)).await;
        1
    });
    info!("Received {result}");
}

/// Or block on a task, without the need of a system parameter.
fn system2() {
    let result = AsyncTask::new(async {
        sleep(Duration::from_millis(1000)).await;
        2
    })
    .blocking_recv();
    info!("Received {result}");
}

pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default(), PanicHandlerPlugin))
        .add_systems(Update, system1)
        .add_systems(Update, system2)
        .run();
}
