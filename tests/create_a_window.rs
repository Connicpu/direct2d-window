extern crate direct2d_window;

#[test]
fn create_a_window() {
    direct2d_window::builder::WindowBuilder::new()
        .class_style(|cls| cls.no_close(true))
        .window_style(|wnd| wnd.overlapped_window_ex(false))
        .title("Test window")
        .build()
        .expect("A basic window should succeed in building");
}
