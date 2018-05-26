extern crate direct2d;
extern crate direct2d_window;
extern crate directwrite;
extern crate dxgi;
extern crate rand;

use direct2d_window::builder::WindowBuilder;
use direct2d_window::event::Event;
use direct2d_window::window::Window;

use direct2d::brush::SolidColorBrush;
use direct2d::enums::DrawTextOptions;
use direct2d::image::Bitmap;
use direct2d::math::*;
use direct2d::{DeviceContext, RenderTarget};
use directwrite::TextFormat;
use dxgi::{PresentFlags, SwapChain};
use rand::Rng;

fn draw(
    context: &mut DeviceContext,
    target: &mut (Bitmap, SwapChain),
    font: &TextFormat,
    dpi: f32,
) {
    let mut rng = rand::thread_rng();
    let brush = SolidColorBrush::create(&context)
        .with_color(0x00_00_00)
        .build()
        .unwrap();

    let psize = target.0.get_pixel_size();
    let size = SizeF::new(psize.width as f32, psize.height as f32);

    context.begin_draw();
    context.set_dpi(dpi * 96.0, dpi * 96.0);
    context.set_target(&target.0);
    context.clear(0xFF_FF_FF);

    let center = Point2F::new(size.width / 2.0 / dpi, size.height / 2.0 / dpi);
    let center = center + Vector2F::new(rng.gen(), rng.gen()) * 10.0;
    let rect = (
        center.x - 200.0,
        center.y - 40.0,
        center.x + 200.0,
        center.y + 40.0,
    );

    context.draw_text(
        "Holy hecking heck!",
        font,
        rect,
        &brush,
        DrawTextOptions::NONE,
    );

    context.end_draw().unwrap();

    target.1.present(1, PresentFlags::NONE).ok();
}

fn main() {
    Window::enable_high_dpi();

    let window = WindowBuilder::new()
        .title("Basic Window example")
        .build()
        .unwrap();

    let mut dpi = window.dpi_scale();

    let mut context = window.get_d2d_context();
    let mut target = window.create_render_target().unwrap();

    let dwrite = directwrite::Factory::new().unwrap();
    let font = TextFormat::create(&dwrite)
        .with_family("Segoe UI")
        .with_size(36.0)
        .build()
        .unwrap();

    for event in window.events() {
        match event {
            Event::CloseRequest => window.close(),
            Event::Quit => break,
            Event::MouseMove { .. } => {}
            Event::DpiChanged { new_dpi } => dpi = new_dpi,
            Event::Resizing { .. } => {}
            Event::Resize { .. } => {
                target = window.create_render_target().unwrap();
                draw(&mut context, &mut target, &font, dpi);
            }
            Event::Paint => {
                draw(&mut context, &mut target, &font, dpi);
            }
            event => {
                println!("{:?}", event);
            }
        }
    }
}
