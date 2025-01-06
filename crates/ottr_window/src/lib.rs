//! # ottr_window
//!
//! Otter's windowing system. Most users will probably use
//! the [`WindowLoop`] type, which implements
//! [`MainLoop`].
//!

extern crate alloc;

use alloc::sync::Arc;

use ottr_main::{MainLoop, MainLoopExit};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

/// `use ottr_window::prelude::*;` to import the most common types, traits and functions.
pub mod prelude {
    pub use crate::WindowLoop;
}

/// An implementation of [`MainLoop`] for [`winit`].
///
/// This will create a window whenever [`MainLoop::run`] is called,
/// and dispatch all events it produces.
#[derive(Debug)]
pub struct WindowLoop {
    event_loop: EventLoop<()>,
}

impl WindowLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().expect("failed to create event loop");

        WindowLoop { event_loop }
    }
}

impl MainLoop for WindowLoop {
    async fn run(self) -> MainLoopExit {
        let Self { event_loop } = self;

        let mut app = App::new();

        if event_loop.run_app(&mut app).is_err() {
            MainLoopExit(1)
        } else {
            MainLoopExit(0)
        }
    }
}

struct App {
    window: Option<Arc<Window>>,
}

impl App {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        self.window = Some(
            event_loop
                .create_window(WindowAttributes::default())
                .expect("failed to create window")
                .into(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
