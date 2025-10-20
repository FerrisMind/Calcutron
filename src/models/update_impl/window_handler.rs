use iced::Task;
extern crate image;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, GWL_STYLE, GetWindowLongA, HWND_NOTOPMOST, HWND_TOPMOST, SWP_FRAMECHANGED,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowLongA, SetWindowPos, WINDOW_STYLE, WS_CAPTION,
    WS_MAXIMIZEBOX, WS_MINIMIZEBOX, SendMessageA, WM_SETICON, ICON_BIG, ICON_SMALL, HICON, LoadImageA, IMAGE_ICON, LR_LOADFROMFILE,
};
use windows::Win32::Foundation::{HINSTANCE, WPARAM};

use crate::models::{calcutron::Calcutron, message::Message};

/// Load Windows HICON directly from ICO file
fn load_hicon_from_file() -> Option<HICON> {
    use windows::core::PCSTR;
    use std::ffi::CString;

    // Create path to icon file
    let icon_path = "static/app_icons/icon.ico";
    let c_path = CString::new(icon_path).ok()?;

    unsafe {
        let hicon_result = LoadImageA(
            Some(HINSTANCE::default()),
            PCSTR(c_path.as_ptr() as *const u8),
            IMAGE_ICON,
            0, // Use default width
            0, // Use default height
            LR_LOADFROMFILE,
        );

        match hicon_result {
            Ok(handle) => {
                if handle.is_invalid() {
                    None
                } else {
                    Some(HICON(handle.0))
                }
            }
            Err(_) => None,
        }
    }
}

pub fn handle_toggle_always_on_top(calcutron: &mut Calcutron) -> Task<Message> {
    let window_id = calcutron.window_id;

    if calcutron.always_on_top {
        // Exiting always-on-top mode - restore previous size and remove always-on-top
        calcutron.always_on_top = false;
        calcutron.compact_mode = false;

        // Try to find window and remove always-on-top
        #[cfg(target_os = "windows")]
        {
            use windows::core::PCSTR;

            let title = "Calcutron";
            let title_bytes = title.as_bytes();
            let mut title_vec = title_bytes.to_vec();
            title_vec.push(0);

            let hwnd = unsafe { FindWindowA(PCSTR::null(), PCSTR(title_vec.as_ptr())) };

            if let Ok(hwnd) = hwnd
                && !hwnd.is_invalid()
            {
                unsafe {
                    let _ = SetWindowPos(
                        hwnd,
                        Some(HWND_NOTOPMOST),
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE,
                    );
                    // Re-enable minimize/maximize buttons in normal mode
                    enable_window_buttons(hwnd);

                    // Re-enable system title bar
                    let current_style = GetWindowLongA(hwnd, GWL_STYLE);
                    let new_style = WINDOW_STYLE(current_style as u32 | WS_CAPTION.0);
                    let _ = SetWindowLongA(hwnd, GWL_STYLE, new_style.0 as i32);

                    // Force window to update its style
                    let _ = SetWindowPos(
                        hwnd,
                        None,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                    );
                }
            }
        }

        // Restore saved window size if available
        let target_size = calcutron
            .saved_window_size
            .unwrap_or(iced::Size::new(320.0, 470.0));
        calcutron.window_size = target_size;
        window_id
            .map(|id| iced::window::resize(id, target_size))
            .unwrap_or_else(Task::none)
    } else {
        // Entering always-on-top mode - save current size and set compact mode
        calcutron.saved_window_size = Some(calcutron.window_size);
        calcutron.always_on_top = true;
        calcutron.compact_mode = true;

        // Try to find window and set always-on-top
        #[cfg(target_os = "windows")]
        {
            use windows::core::PCSTR;

            let title = "Calcutron";
            let title_bytes = title.as_bytes();
            let mut title_vec = title_bytes.to_vec();
            title_vec.push(0);

            let hwnd = unsafe { FindWindowA(PCSTR::null(), PCSTR(title_vec.as_ptr())) };

            if let Ok(hwnd) = hwnd
                && !hwnd.is_invalid()
            {
                unsafe {
                    let _ = SetWindowPos(
                        hwnd,
                        Some(HWND_TOPMOST),
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE,
                    );
                    // Disable minimize/maximize buttons in compact mode
                    disable_window_buttons(hwnd);

                    // Disable system title bar for custom chrome, but keep resize border
                    let current_style = GetWindowLongA(hwnd, GWL_STYLE);
                    let new_style = WINDOW_STYLE(current_style as u32 & !(WS_CAPTION.0)); // Remove caption, keep thick frame
                    let _ = SetWindowLongA(hwnd, GWL_STYLE, new_style.0 as i32);

                    // Force window to update its style
                    let _ = SetWindowPos(
                        hwnd,
                        None,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                    );
                }
            }
        }

        // Set compact window size
        let compact_size = iced::Size::new(250.0, 280.0);
        calcutron.window_size = compact_size;
        window_id
            .map(|id| iced::window::resize(id, compact_size))
            .unwrap_or_else(Task::none)
    }
}

fn disable_window_buttons(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let current_style = GetWindowLongA(hwnd, GWL_STYLE);
        let new_style = WINDOW_STYLE(current_style as u32 & !(WS_MINIMIZEBOX.0 | WS_MAXIMIZEBOX.0));
        let _ = SetWindowLongA(hwnd, GWL_STYLE, new_style.0 as i32);
    }
}

fn enable_window_buttons(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let current_style = GetWindowLongA(hwnd, GWL_STYLE);
        let new_style = WINDOW_STYLE(current_style as u32 | WS_MINIMIZEBOX.0 | WS_MAXIMIZEBOX.0);
        let _ = SetWindowLongA(hwnd, GWL_STYLE, new_style.0 as i32);
    }
}

pub fn handle_window_event(
    calcutron: &mut Calcutron,
    id: iced::window::Id,
    event: iced::window::Event,
) -> Task<Message> {
    // Handle window events
    calcutron.window_id = Some(id);
    match event {
        iced::window::Event::Opened { .. } => {
            // Set application icon when window opens
            if calcutron.app_icon.is_some() {
                #[cfg(target_os = "windows")]
                {
                    use windows::core::PCSTR;
                    use windows::Win32::Foundation::LPARAM;

                    // Find the window handle
                    let title = "Calcutron";
                    let title_bytes = title.as_bytes();
                    let mut title_vec = title_bytes.to_vec();
                    title_vec.push(0);

                    let hwnd = unsafe { FindWindowA(PCSTR::null(), PCSTR(title_vec.as_ptr())) };

                    if let Ok(hwnd) = hwnd && !hwnd.is_invalid() {
                        // Load HICON directly from file
                        if let Some(hicon) = load_hicon_from_file() {
                            // Set both big and small icons
                            unsafe {
                                SendMessageA(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(hicon.0 as isize));
                                SendMessageA(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(hicon.0 as isize));
                            }
                            eprintln!("Application icon successfully set on window");
                        } else {
                            eprintln!("Failed to load HICON from file");
                        }
                    } else {
                        eprintln!("Could not find window handle to set icon");
                    }
                }
            }
            Task::none()
        }
        iced::window::Event::CloseRequested => Task::none(),
        iced::window::Event::Resized(size) => {
            calcutron.window_size = size;
            Task::none()
        }
        _ => Task::none(),
    }
}
