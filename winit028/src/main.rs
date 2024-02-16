use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();

    event_loop.run(move |event, _elwt, control_flow| match event {
        Event::RedrawRequested(..) => {
            window.request_redraw();
        }
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            _ => (),
        },
        _ => (),
    });
}
