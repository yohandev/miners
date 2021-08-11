use winit::event_loop::{ ControlFlow, EventLoop };
use winit::window::WindowBuilder;
use winit::event::*;

fn main()
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow|
    {
        match event
        {
            Event::WindowEvent { ref event, window_id }
                if window_id == window.id()
                && matches!(event, WindowEvent::CloseRequested) =>
            {
                *control_flow = ControlFlow::Exit
            },
            _ => { }
        }
    })
}
