use event::Event;
use internals::win32_helpers::WindowClass;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::any::Any;
use std::ptr;

use winapi::*;

pub struct WindowInner {
    pub hwnd: Cell<HWND>,
    pub events: RefCell<VecDeque<Event>>,
    pub dpi_scale: f32,
    pub panic: Cell<Option<Box<Any + Send>>>,
    pub wndclass: WindowClass,
}

impl WindowInner {
    pub fn new(class: WindowClass) -> WindowInner {
        WindowInner {
            hwnd: Cell::new(ptr::null_mut()),
            events: RefCell::new(VecDeque::new()),
            dpi_scale: 1.0, // TODO
            panic: Cell::new(None),
            wndclass: class,
        }
    }
}
