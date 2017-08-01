use event::Event;
use internals::data::WindowInner;
use internals::win32_helpers;

pub struct Window {
    inner: Box<WindowInner>,
}

impl Window {
    pub(crate) fn new(inner: Box<WindowInner>) -> Self {
        Window { inner: inner }
    }

    pub unsafe fn get_hwnd(&self) -> ::winapi::HWND {
        self.inner.hwnd.get()
    }

    /// It is highly recommended that you call this method before doing anything
    /// else unless there is a strong technical reason that you hate people with
    /// nice monitors :(
    pub fn enable_high_dpi() {
        ::internals::dpi::enable_dpi();
    }

    pub fn dpi_scale(&self) -> f32 {
        self.inner.dpi_scale.get()
    }

    pub fn poll_events(&self) -> EventsIter {
        EventsIter { inner: &self.inner }
    }

    pub fn close(&self) {
        unsafe {
            ::user32::DestroyWindow(self.inner.hwnd.get());
        }
    }
}

pub struct EventsIter<'a> {
    inner: &'a WindowInner,
}

impl<'a> Iterator for EventsIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        if self.inner.events.borrow().is_empty() {
            win32_helpers::process_message(self.inner);
        }

        self.inner.events.borrow_mut().pop_front()
    }
}
