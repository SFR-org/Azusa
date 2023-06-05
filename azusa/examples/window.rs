use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use azusa::{Color, Object, Surface, WindowSurface};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut surface = WindowSurface::new(&window);
    let command = [Object::Clear(Color::White)];

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
                surface.resize();
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