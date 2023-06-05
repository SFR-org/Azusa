use raw_window_handle::HasRawWindowHandle;

pub mod error;
pub(crate) mod window;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Color {
    White
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Object {
    Clear(Color)
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
    pub fn new(handle: &impl HasRawWindowHandle) -> Self {
        Self {
            inner: window::get_graphics_instance(handle),
        }
    }

    pub fn submit(&mut self, obj: &[Object]) {
        self.inner.submit(obj);
    }

    pub fn resize(&mut self) {
        self.inner.resize(0,0);
    }
}