use error::DResult;
use event::Event;
use internals::target;
use internals::win32_helpers::WindowClass;

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::ptr;

use direct2d::{self, DeviceContext, Factory};
use direct3d11::device::Device as D3D11Device;
use dxgi::Factory as DXGIFactory;
use winapi::shared::windef::HWND;

pub struct WindowInner {
    pub hwnd: Cell<HWND>,
    pub events: RefCell<VecDeque<Event>>,
    pub dpi_scale: Cell<f32>,
    pub panic: Cell<Option<Box<Any + Send>>>,
    pub wndclass: WindowClass,
    pub d2d_factory: Factory,
    pub d2d_context: DeviceContext,
    pub d3d_device: D3D11Device,
    pub dxgi_factory: DXGIFactory,
}

impl WindowInner {
    pub fn create(class: WindowClass, factory: Factory) -> DResult<WindowInner> {
        let d3d = target::default_d3d()?;
        let dev = direct2d::Device::create(&factory, &d3d.as_dxgi())?;
        let ctx = DeviceContext::create(&dev, false)?;
        let dxgi = d3d.as_dxgi().get_adapter()?.get_factory();

        let inner = WindowInner {
            hwnd: Cell::new(ptr::null_mut()),
            events: RefCell::new(VecDeque::new()),
            dpi_scale: Cell::new(1.0),
            panic: Cell::new(None),
            wndclass: class,
            d2d_factory: factory,
            d2d_context: ctx,
            d3d_device: d3d,
            dxgi_factory: dxgi,
        };
        Ok(inner)
    }
}
