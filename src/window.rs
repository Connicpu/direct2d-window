use error::DResult;
use event::Event;
use internals::data::WindowInner;
use internals::target;
use internals::win32_helpers;

use std::mem;

use direct2d::image::Bitmap;
use direct2d::{DeviceContext, Factory};
use dxgi::SwapChain;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use windows_dpi as dpi;

pub struct Window {
    inner: Box<WindowInner>,
}

impl Window {
    pub(crate) fn new(inner: Box<WindowInner>) -> Self {
        Window { inner: inner }
    }

    pub unsafe fn get_hwnd(&self) -> HWND {
        self.inner.hwnd.get()
    }

    pub fn get_d2d_factory(&self) -> Factory {
        self.inner.d2d_factory.clone()
    }

    pub fn get_d2d_context(&self) -> DeviceContext {
        self.inner.d2d_context.clone()
    }

    /// It is highly recommended that you call this method before doing anything
    /// else unless there is a strong technical reason that you hate people with
    /// nice monitors :(
    pub fn enable_high_dpi() {
        dpi::enable_dpi();
    }

    pub fn dpi_scale(&self) -> f32 {
        self.inner.dpi_scale.get()
    }

    pub fn poll_events(&self) -> EventsIter {
        while win32_helpers::peek_message(&self.inner) {}
        EventsIter { inner: &self.inner }
    }

    pub fn events(&self) -> BlockingEventsIter {
        BlockingEventsIter { inner: &self.inner }
    }

    pub fn close(&self) {
        unsafe {
            DestroyWindow(self.inner.hwnd.get());
        }
    }

    pub fn get_client_size(&self) -> (i32, i32) {
        unsafe {
            let hwnd = self.get_hwnd();
            let mut rect = mem::uninitialized();
            GetClientRect(hwnd, &mut rect);

            (rect.right - rect.left, rect.bottom - rect.top)
        }
    }

    pub fn create_render_target(&self) -> DResult<(Bitmap, SwapChain)> {
        let dxgi = &self.inner.dxgi_factory;
        let d3d = &self.inner.d3d_device;
        let hwnd = self.inner.hwnd.get();
        let ctx = &self.inner.d2d_context;

        let swapchain = target::create_swapchain(dxgi, d3d, hwnd)?;
        let bitmap = target::create_backing(&swapchain, ctx)?;

        Ok((bitmap, swapchain))
    }
}

pub struct EventsIter<'a> {
    inner: &'a WindowInner,
}

impl<'a> Iterator for EventsIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.inner.events.borrow_mut().pop_front()
    }
}

pub struct BlockingEventsIter<'a> {
    inner: &'a WindowInner,
}

impl<'a> Iterator for BlockingEventsIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        while self.inner.events.borrow().is_empty() {
            win32_helpers::wait_message(&self.inner);
        }
        self.inner.events.borrow_mut().pop_front()
    }
}
