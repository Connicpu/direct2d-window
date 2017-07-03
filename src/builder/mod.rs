use internals::win32_helpers::{create_window, create_window_class};
use window::Window;

use winapi::HRESULT;

pub use self::styles::{WindowStyle, WindowClassStyle};

pub mod styles;

#[derive(Default)]
pub struct WindowProperties {
    pub style: WindowStyle,
    pub title: Option<String>,
}

impl WindowProperties {
    pub fn new() -> WindowProperties {
        Default::default()
    }
}

#[derive(Default)]
pub struct WindowBuilder {
    pub class_style: styles::WindowClassStyle,
    pub window_props: WindowProperties,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn class_style<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(styles::WindowClassStyle) -> styles::WindowClassStyle,
    {
        self.class_style = func(self.class_style);
        self
    }

    pub fn window_style<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(styles::WindowStyle) -> styles::WindowStyle,
    {
        self.window_props.style = func(self.window_props.style);
        self
    }

    pub fn title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.window_props.title = Some(title.into());
        self
    }

    pub fn build(&self) -> Result<Window, WindowError> {
        use self::WindowError::*;

        let class = create_window_class(self.class_style)
            .map_err(ClassRegistration)?;
        let window = create_window(class, &self.window_props)
            .map_err(WindowCreation)?;

        Ok(window)
    }
}

#[derive(Debug)]
pub enum WindowError {
    ClassRegistration(HRESULT),
    WindowCreation(HRESULT),
}
