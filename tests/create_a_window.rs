extern crate direct2d;
extern crate direct2d_window;
extern crate dxgi;

use direct2d::brush::SolidColorBrush;
use direct2d::RenderTarget;

#[test]
fn create_a_window() {
    let window = direct2d_window::builder::WindowBuilder::new()
        .class_style(|cls| cls.no_close(true))
        .window_style(|wnd| wnd.overlapped_window_ex(false))
        .title("Test window")
        .build()
        .expect("A basic window should succeed in building");

    let target = window.create_render_target().unwrap();
    let swapchain = window.get_swap_chain().unwrap();

    let size = target.get_size();

    let mut context = window.get_d2d_context();
    context.begin_draw();
    context.set_target(&target);
    context.clear(0xFF_FF_FF);
    context.end_draw().unwrap();

    let brush = SolidColorBrush::create(&context)
        .with_color((0xFF_00_00, 1.1 / 40.0))
        .build()
        .unwrap();

    for _ in 0..40 {
        for _evt in window.poll_events() {}

        context.begin_draw();
        context.set_target(&target);
        context.fill_rectangle((0.0, 0.0, size.width, size.height), &brush);
        context.end_draw().unwrap();

        swapchain.present(1, dxgi::PresentFlags::NONE).ok();
    }
}
