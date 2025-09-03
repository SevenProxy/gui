use winit::window::{
    Window,
    WindowId,
};
use winit::event::WindowEvent;
use winit::event_loop::{
    ActiveEventLoop,
    EventLoop,
    ControlFlow,
};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Setting window
        // Window size setting
        // https://docs.rs/winit/latest/winit/window/struct.WindowAttributes.html
        let pixels_logical = LogicalSize::new(128.0, 128.0);
        let window_attributes = Window::default_attributes()
            .with_title("Pitou")
            .with_inner_size(pixels_logical);
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // Event validation window
        // https://docs.rs/winit/latest/winit/window/struct.WindowAttributes.html
        match event {
            WindowEvent::CloseRequested => {
                let message_close_window: &str = "Window exit.";
                println!("{}", message_close_window);
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => {},
        }
    }
}

fn main() {
    // Create event loop
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    
    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);


    let mut app = App::default();
    event_loop.run_app(&mut app);
}
