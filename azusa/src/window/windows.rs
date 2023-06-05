use std::ffi::c_void;
use std::ptr::null_mut;
use gdiplus_sys2::{ARGB, GdiplusStartupInput, GpGraphics};
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::RGB;
use winapi::um::winuser::{GetDC, ReleaseDC};
use crate::{Object, Surface};

pub struct WindowsSurface {
    token: ULONG_PTR,
    hdc: HDC,
    hwnd: HWND,
    graphics: *mut GpGraphics
}

impl WindowsSurface {
    pub fn new(hwnd: *mut c_void) -> Self {
        let hwnd = hwnd as HWND;
        let mut token = 0;
        let input = GdiplusStartupInput {
            GdiplusVersion: 1,
            DebugEventCallback: None,
            SuppressBackgroundThread: 0,
            SuppressExternalCodecs: 0,
        };

        let status = unsafe { gdiplus_sys2::GdiplusStartup(&mut token, &input, null_mut()) };
        if status != gdiplus_sys2::Status_Ok {
            panic!("Can't initialize GDI+");
        }

        let hdc = unsafe { GetDC(hwnd) };
        let mut graphics = null_mut();
        let status = unsafe { gdiplus_sys2::GdipCreateFromHDC(hdc,&mut graphics) };
        if status != gdiplus_sys2::Status_Ok {
            panic!("Failed to create Graphics object");
        }

        Self {
            token,
            hdc,
            hwnd,
            graphics
        }
    }
}

impl Surface for WindowsSurface {
    fn resize(&mut self, width: u32, height: u32) {
        let hdc = unsafe { GetDC(self.hwnd) };
        let mut graphics = null_mut();
        let status = unsafe { gdiplus_sys2::GdipCreateFromHDC(hdc,&mut graphics) };
        if status != gdiplus_sys2::Status_Ok {
            panic!("Failed to create Graphics object");
        }

        self.graphics = graphics;
    }

    fn submit(&mut self, obj: &[Object]) {
        for i in obj {
            match i {
                Object::Clear(color) => {
                    unsafe {
                        gdiplus_sys2::GdipGraphicsClear(self.graphics, gdiplus_sys2::Color_Aqua as ARGB);
                    }
                }
            }
        }
    }
}

impl Drop for WindowsSurface {
    fn drop(&mut self) {
        unsafe {
            gdiplus_sys2::GdipDeleteGraphics(self.graphics);
            ReleaseDC(self.hwnd,self.hdc);
            gdiplus_sys2::GdiplusShutdown(self.token);
        }
    }
}