use std::ffi::c_void;
use std::ptr::null_mut;
use gdiplus_sys2::{ARGB, GdiplusStartupInput, GpBrush, GpGraphics, REAL};
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::BYTE;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{GetBValue, GetGValue, GetRValue, RGB};
use winapi::um::winuser::{GetDC, ReleaseDC};
use crate::{Color, Object, Surface};

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

        unsafe {
            gdiplus_sys2::GdipDeleteGraphics(self.graphics);
        }

        self.graphics = graphics;
    }

    fn submit(&mut self, obj: &[Object]) {
        for i in obj {
            match i {
                Object::Clear(color) => {
                    unsafe {
                        gdiplus_sys2::GdipGraphicsClear(self.graphics, (*color).into());
                    }
                }

                Object::FillRectangle(x,y,width,height,color) => {
                    unsafe {
                        let mut solid = std::mem::zeroed();
                        let status = gdiplus_sys2::GdipCreateSolidFill((*color).into(),&mut solid);
                        if status != gdiplus_sys2::Status_Ok {
                            panic!("Can't create GpBrush");
                        }
                        gdiplus_sys2::GdipFillRectangle(self.graphics, solid as *mut GpBrush, *x as REAL,*y as REAL,*width as REAL,*height as REAL);
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

fn get_color(r: u8,g: u8,b: u8) -> ARGB {
    let colorref = RGB(r,g,b);
    let r = GetRValue(colorref) as u32;
    let g = GetGValue(colorref) as u32;
    let b = GetBValue(colorref) as u32;
    (255u32 << 24) | (r << 16) | (g << 8) | b
}

impl Into<ARGB> for Color {
    fn into(self) -> ARGB {
        match self {
            Color::Black => get_color(0,0,0),
            Color::Silver => get_color(192,192,192),
            Color::Gray => get_color(128,128,128),
            Color::White => get_color(255,255,255),
            Color::Maroon => get_color(128,0,0),
            Color::Red => get_color(255,0,0),
            Color::Purple => get_color(128,0,128),
            Color::Fuchsia => get_color(255,0,255),
            Color::Green => get_color(0,128,0),
            Color::Lime => get_color(0,255,0),
            Color::Olive => get_color(128,128,0),
            Color::Yellow => get_color(255,255,0),
            Color::Navy => get_color(0,0,128),
            Color::Blue => get_color(0,0,255),
            Color::Teal => get_color(0,128,128),
            Color::Aqua => get_color(0,255,255),
            Color::Rgb(r,g,b) => get_color(r,g,b),
        }
    }
}