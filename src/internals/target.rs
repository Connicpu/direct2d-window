use error::DResult;

use direct2d::enums::BitmapOptions;
use direct2d::image::Bitmap;
use direct2d::DeviceContext;
use direct3d11::flags::CreateDeviceFlags;
use direct3d11::flags::DriverType;
use direct3d11::Device;
use direct3d11::Texture2D;
use dxgi::Factory as DxgiFactory;
use dxgi::{SwapChain, UsageFlags};
use winapi::shared::windef::HWND;

pub fn default_d3d() -> DResult<Device> {
    let flags = if cfg!(debug_assertions) {
        CreateDeviceFlags::BGRA_SUPPORT | CreateDeviceFlags::DEBUG
    } else {
        CreateDeviceFlags::BGRA_SUPPORT
    };

    let (_, dev, _) = Device::create()
        .with_driver_type(DriverType::Hardware)
        .with_flags(flags)
        .build()?;
    Ok(dev)
}

pub fn create_swapchain(factory: &DxgiFactory, device: &Device, hwnd: HWND) -> DResult<SwapChain> {
    let flags =
        UsageFlags::BACK_BUFFER | UsageFlags::RENDER_TARGET_OUTPUT | UsageFlags::SHADER_INPUT;
    let swapchain = SwapChain::create_hwnd(&factory, &device.as_dxgi())
        .buffer_usage(flags)
        .hwnd(hwnd)
        .build()?;
    Ok(swapchain)
}

pub fn create_backing(swapchain: &SwapChain, ctx: &DeviceContext) -> DResult<Bitmap> {
    let tex: Texture2D = swapchain.get_buffer(0)?;
    let bitmap = Bitmap::create(&ctx)
        .with_dxgi_surface(&tex.as_dxgi())
        .with_options(BitmapOptions::TARGET)
        .build()?;

    Ok(bitmap)
}
