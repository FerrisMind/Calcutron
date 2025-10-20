use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE,
};

use crate::models::{calcutron::Calcutron, message::Message};

/// Custom event loop application for Calcutron
pub struct CalcutronApp {
    state: Calcutron,
    window: Option<Window>,
}

impl CalcutronApp {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let mut app = CalcutronApp {
            state: Calcutron::default(),
            window: None,
        };

        event_loop.run_app(&mut app)?;
        Ok(())
    }

    fn set_window_always_on_top(&self, always_on_top: bool) {
        if let Some(window) = &self.window {
            if let Ok(window_handle) = window.window_handle() {
                if let RawWindowHandle::Win32(handle) = window_handle.as_raw() {
                    let hwnd_insert_after = if always_on_top {
                        HWND_TOPMOST
                    } else {
                        HWND_NOTOPMOST
                    };

                    unsafe {
                        let _ = SetWindowPos(
                            windows::Win32::Foundation::HWND(handle.hwnd.get() as _),
                            Some(hwnd_insert_after),
                            0,
                            0,
                            0,
                            0,
                            SWP_NOMOVE | SWP_NOSIZE,
                        );
                    }
                }
            }
        }
    }
}

impl ApplicationHandler<()> for CalcutronApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("Calcutron")
            .with_inner_size(winit::dpi::LogicalSize::new(320.0, 470.0))
            .with_min_inner_size(winit::dpi::LogicalSize::new(320.0, 470.0))
            .with_transparent(true);

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Handle redraw
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, message: Message) {
        match message {
            Message::ToggleAlwaysOnTop => {
                self.state.always_on_top = !self.state.always_on_top;
                self.set_window_always_on_top(self.state.always_on_top);
            }
            Message::WindowEvent(event) => {
                // Handle other messages if needed
                let _ = self.state.update(Message::WindowEvent(event));
            }
            _ => {
                let _ = self.state.update(message);
            }
        }
    }
}
