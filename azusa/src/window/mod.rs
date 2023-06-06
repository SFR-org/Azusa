use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::Surface;

pub mod windows;

#[doc(hidden)]
pub fn get_graphics_instance(handle: &impl HasRawWindowHandle,width: u32,height: u32) -> Box<dyn Surface> {
    let surface = match handle.raw_window_handle() {
        RawWindowHandle::UiKit(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::AppKit(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Orbital(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Xlib(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Xcb(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Wayland(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Drm(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Gbm(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Win32(handle) => {
            let surface = windows::WindowsSurface::new(handle.hwnd, width as i32, height as i32);
            Box::new(surface)
        }
        RawWindowHandle::WinRt(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Web(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::AndroidNdk(_) => {
            panic!("Sorry, this platform is not supported");
        }
        RawWindowHandle::Haiku(_) => {
            panic!("Sorry, this platform is not supported");
        }
        _ => {
            panic!("Unknown platform.");
        }
    };

    surface
}