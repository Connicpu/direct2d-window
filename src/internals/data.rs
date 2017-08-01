use event::Event;
use internals::win32_helpers::WindowClass;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::any::Any;
use std::ptr;

use direct2d::Factory;
use winapi::*;

pub struct WindowInner {
    pub hwnd: Cell<HWND>,
    pub events: RefCell<VecDeque<Event>>,
    pub dpi_scale: Cell<f32>,
    pub panic: Cell<Option<Box<Any + Send>>>,
    pub wndclass: WindowClass,
    pub d2d_factory: Factory,
}

impl WindowInner {
    pub fn new(class: WindowClass, factory: Factory) -> WindowInner {
        WindowInner {
            hwnd: Cell::new(ptr::null_mut()),
            events: RefCell::new(VecDeque::new()),
            dpi_scale: Cell::new(1.0),
            panic: Cell::new(None),
            wndclass: class,
            d2d_factory: factory,
        }
    }
}
