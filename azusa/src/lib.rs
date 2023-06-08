use raw_window_handle::HasRawWindowHandle;

pub mod error;
pub(crate) mod window;

/// Color Definition.
/// Mainly used to specify the drawing color
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

/// # Method
/// Definition of methods for drawing on surfaces.
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Method {
    /// # Clear method.
    /// ### Arguments
    /// Clear(color: Color)
    /// color: Specifies the color to be cleared
    Clear(Color),
    /// # DrawRectangle method.
    /// ### Arguments
    /// Clear(x: u32,y: u32,height: u32,width: u32,thickness: u32,color: Color)
    DrawRectangle(u32,u32,u32,u32,u32,Color),
    /// # FillRectangle method.
    /// ### Arguments
    /// Clear(x: u32,y: u32,width: u32,height: u32,color: Color)
    FillRectangle(u32,u32,u32,u32,Color)
}

pub trait Surface {
    /// Resizes the surface.
    fn resize(&mut self,width:u32,height:u32);
    /// Executes the passed drawing method **from the beginning of the array**
    fn submit(&mut self, obj: Vec<Method>);
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

    #[inline]
    pub fn submit(&mut self, obj: &mut Vec<Method>) {
        optimize(obj);
        self.inner.submit(obj);
    }

    #[inline]
    pub fn resize(&mut self,width: u32,height: u32) {
        self.inner.resize(width,height);
    }
}

fn optimize(methods: &mut Vec<Method>) {
    static mut CNT:u32 = 0;
        let index = methods.iter().position(|&item| 
        
        match item {
            Method::Fill => false,
            Method::Clear(c) => {
                if unsafe { CNT == 0 } {
                    unsafe { CNT += 1 };
                    return false;
                }
                unsafe {
                CNT += 1;}
                item == Method::Clear(c)
                
            }
        }).unwrap();
        
        for i in 0..index {
            methods.remove(0);
        }
        unsafe {
            CNT = 0;
        }
        println!("{}",index);
    }