use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use azusa::{Color, Method, Surface, WindowSurface};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1280,720))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();

    let mut surface = WindowSurface::new(&window,size.width,size.height);

    let command = [
        Method::Clear(Color::Red),
        Method::FillRectangle(100, 100, 200, 200, Color::White)
    ];

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                window_id,
            } => {
                let size = window.inner_size();
                surface.resize(size.width,size.height);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                surface.submit(&command);
            }
            _ => (),
        }
    });
}