use event::Event;
use internals::win32_helpers::WindowClass;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::any::Any;

use winapi::*;

pub struct WindowInner {
    pub hwnd: Cell<HWND>,
    pub events: RefCell<VecDeque<Event>>,
    pub dpi_scale: f32,
    pub panic: Cell<Option<Box<Any>>>,
    pub wndclass: WindowClass,
}
