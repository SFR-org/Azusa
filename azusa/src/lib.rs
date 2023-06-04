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
    fn draw_object(&mut self,obj: &[Object]);
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
}

impl Surface for WindowSurface {
    fn draw_object(&mut self, obj: &[Object]) {
        self.inner.draw_object(obj);
    }
}