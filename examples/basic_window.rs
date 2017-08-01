extern crate direct2d_window;

use direct2d_window::builder::WindowBuilder;
use direct2d_window::event::Event;
use direct2d_window::window::Window;

fn main() {
    Window::enable_high_dpi();

    let window = WindowBuilder::new()
        .title("Basic Window example")
        .build()
        .unwrap();

    'outer: loop {
        for event in window.poll_events() {
            match event {
                Event::CloseRequest => window.close(),
                Event::Quit => break 'outer,
                event => {
                    println!("{:?}", event);
                }
            }
        }

        std::thread::sleep_ms(16);
    }
}
