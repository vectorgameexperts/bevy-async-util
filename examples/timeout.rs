use bevy::prelude::*;
use bevy_async_task::AsyncTask;
use std::time::Duration;

/// Use a timeout
fn system() {
    let _timeout = AsyncTask::new_with_timeout(
        Duration::from_millis(1000),
        async_std::future::pending::<()>(),
    )
    .blocking_recv()
    .unwrap_err();

    println!("Timeout!");
}

pub fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Update, system)
        .run();
}
