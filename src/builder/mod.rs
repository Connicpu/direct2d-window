use error::DResult;
use internals::win32_helpers::{create_window, create_window_class};
use window::Window;

use direct2d::Factory;

pub use self::styles::{WindowClassStyle, WindowStyle};

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
    pub d2d_factory: Option<Factory>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// If you don't call this method, a factory will be created for you
    pub fn with_factory(&mut self, factory: Factory) -> &mut Self {
        self.d2d_factory = Some(factory);
        self
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

    pub fn build(&self) -> DResult<Window> {
        ::windows_dpi::enable_dpi();

        let factory = self.d2d_factory
            .clone()
            .map(Ok)
            .unwrap_or_else(|| Factory::new())?;

        let class = create_window_class(self.class_style)?;
        let window = create_window(class, &self.window_props, factory)?;

        Ok(window)
    }
}
