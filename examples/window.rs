//! This example shows how to use `ottr_window` to create a window.

use ottr::prelude::*;

#[tokio::main]
async fn main() -> MainLoopExit {
    let loop_ = WindowLoop::new();

    loop_.run().await
}
