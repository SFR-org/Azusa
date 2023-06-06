use raw_window_handle::HasRawWindowHandle;

pub mod error;
pub(crate) mod window;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Color {
    Black,
    Silver,
    Gray,
    White,
    Maroon,
    Red,
    Purple,
    Fuchsia,
    Green,
    Lime,
    Olive,
    Yellow,
    Navy,
    Blue,
    Teal,
    Aqua,
    Rgb(u8,u8,u8)
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Object {
    Clear(Color),
    FillRectangle(u32,u32,u32,u32,Color)
}

pub trait Surface {
    fn resize(&mut self,width:u32,height:u32);
    fn submit(&mut self, obj: &[Object]);
}

impl WindowSurface {

}

pub struct WindowSurface {
    inner: Box<dyn Surface>
}

impl WindowSurface {
    pub fn new(handle: &impl HasRawWindowHandle,width: u32,height: u32) -> Self {
        Self {
            inner: window::get_graphics_instance(handle,width,height),
        }
    }

    pub fn submit(&mut self, obj: &[Object]) {
        self.inner.submit(obj);
    }

    pub fn resize(&mut self,width: u32,height: u32) {
        self.inner.resize(width,height);
    }
}