use std::ffi::{c_int, c_void};
use std::ptr::null_mut;
use std::time::Duration;
use gdiplus_sys2::{ARGB, Bitmap, GdiplusStartupInput, GpBrush, GpGraphics, GpImage, REAL};
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::BYTE;
use winapi::shared::windef::{HBITMAP, HDC, HGDIOBJ, HWND, RECT};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetBValue, GetGValue, GetRValue, RGB, SelectObject, SRCCOPY, SwapBuffers};
use winapi::um::winuser::{GetClientRect, GetDC, ReleaseDC};
use crate::{Color, Method, Surface};

#[doc(hidden)]
pub struct WindowsSurface {
    token: ULONG_PTR,
    buffer: *mut GpGraphics,
    buffer_bitmap: HBITMAP,
    bitmap_old: HGDIOBJ,
    hwnd: HWND,
    hdc: HDC,
    buffer_dc: HDC
}

impl WindowsSurface {
    pub fn new(hwnd: *mut c_void,width: i32,height:i32) -> Self {
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
        let buffer_dc = unsafe { CreateCompatibleDC(hdc) };
        let buffer_bitmap = unsafe { CreateCompatibleBitmap(hdc,width,height) };
        let bitmap_old = unsafe { SelectObject(buffer_dc,buffer_bitmap as HGDIOBJ) };

        let mut buffer = null_mut();
        let status = unsafe { gdiplus_sys2::GdipCreateFromHDC(buffer_dc,&mut buffer) };

        Self {
            token,
            buffer,
            buffer_bitmap,
            bitmap_old,
            hwnd,
            hdc,
            buffer_dc
        }
    }
}

impl Surface for WindowsSurface {
    fn resize(&mut self, width: u32, height: u32) {
        if self.buffer != null_mut() {
            unsafe {
                gdiplus_sys2::GdipDeleteGraphics(self.buffer);
            }
        }

        unsafe {
            SelectObject(self.buffer_dc,self.bitmap_old);
            DeleteObject(self.buffer_bitmap as HGDIOBJ);
            DeleteDC(self.buffer_dc);
        }

        let hdc = unsafe { GetDC(self.hwnd) };
        let buffer_dc = unsafe { CreateCompatibleDC(hdc) };
        let buffer_bitmap = unsafe { CreateCompatibleBitmap(hdc, width as c_int, height as c_int) };
        let bitmap_old = unsafe { SelectObject(buffer_dc,buffer_bitmap as HGDIOBJ) };

        let mut buffer = null_mut();
        let status = unsafe { gdiplus_sys2::GdipCreateFromHDC(buffer_dc,&mut buffer) };

        self.hdc = hdc;
        self.buffer_dc = buffer_dc;
        self.buffer = buffer;
        self.buffer_bitmap = buffer_bitmap;
    }

    fn submit(&mut self, obj: &[Method]) {
        for i in obj {
            match i {
                Method::Clear(color) => {
                    unsafe {
                        gdiplus_sys2::GdipGraphicsClear(self.buffer, (*color).into());
                    }
                }

                Method::DrawRectangle(x, y, width, height, thickness, color) => {
                    unsafe {
                        let mut pen = null_mut();
                        let status = gdiplus_sys2::GdipCreatePen1((*color).into(),*thickness as REAL,0,&mut pen);
                        gdiplus_sys2::GdipDrawRectangle(self.buffer,pen,*x as REAL,*y as REAL,*width as REAL,*height as REAL);
                    }
                }

                Method::FillRectangle(x, y, width, height, color) => {
                    unsafe {
                        let mut solid = std::mem::zeroed();
                        let status = gdiplus_sys2::GdipCreateSolidFill((*color).into(),&mut solid);
                        if status != gdiplus_sys2::Status_Ok {
                            panic!("Can't create GpBrush");
                        }

                        gdiplus_sys2::GdipFillRectangle(self.buffer, solid as *mut GpBrush, *x as REAL,*y as REAL,*width as REAL,*height as REAL);
                    }
                }
            }
        }

        unsafe {
            BitBlt(self.hdc, 0, 0, get_client_width(self.hwnd),get_client_height(self.hwnd),self.buffer_dc, 0, 0, SRCCOPY);
        }

        // unsafe {
        //     let hdc = GetDC(self.hwnd);
        //     let mut graphics = null_mut();
        //     let status = gdiplus_sys2::GdipCreateFromHDC(hdc,&mut graphics);
        //     BitBlt(hdc, 0, 0, 1280, 720, self.buffer_dc, 0, 0, SRCCOPY);
        // }
    }
}

impl Drop for WindowsSurface {
    fn drop(&mut self) {
        unsafe {
            gdiplus_sys2::GdipDeleteGraphics(self.buffer);
            ReleaseDC(self.hwnd,self.buffer_dc);
            gdiplus_sys2::GdiplusShutdown(self.token);
        }
    }
}

#[doc(hidden)]
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

#[doc(hidden)]
fn get_client_width(hwnd: HWND) -> i32 {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetClientRect(hwnd,&mut rect);
    }

    rect.right - rect.left
}

#[doc(hidden)]
fn get_client_height(hwnd: HWND) -> i32 {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetClientRect(hwnd,&mut rect);
    }

    rect.bottom - rect.top
}