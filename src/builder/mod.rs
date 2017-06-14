use internals::win32_helpers::{create_window, create_window_class};
use window::Window;

use winapi::HRESULT;

pub mod styles;

#[derive(Default)]
pub struct WindowBuilder {
    class_style: styles::WindowClassStyle,
    window_style: styles::WindowStyle,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn class_style<F>(&mut self, func: F) -> &mut Self
        where F: FnOnce(styles::WindowClassStyle) -> styles::WindowClassStyle
    {
        self.class_style = func(self.class_style);
        self
    }

    pub fn window_style<F>(&mut self, func: F) -> &mut Self
        where F: FnOnce(styles::WindowStyle) -> styles::WindowStyle
    {
        self.window_style = func(self.window_style);
        self
    }

    pub fn build(&self) -> Result<Window, WindowError> {
        use self::WindowError::*;

        let class = create_window_class(self.class_style).map_err(ClassRegistration)?;
        let window = create_window(class, self.window_style).map_err(WindowCreation)?;

        unimplemented!();
    }
}

#[derive(Debug)]
pub enum WindowError {
    ClassRegistration(HRESULT),
    WindowCreation(HRESULT),
}
