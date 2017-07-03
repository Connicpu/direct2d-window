/// Window events for things that happen to your direct2d window.
/// All pixel values are measured in DIPs, which means their values
/// directly translate into the coordinates for direct2d. You can
/// get the current DPI scale from the Window if you need to convert
/// back to pixel coordinates.
#[derive(Clone, PartialEq)]
pub enum Event {
    /// This event is raised when the user requests that the window be closed,
    /// either via the X button or pressing Alt+F4
    CloseRequest,
    /// This message indicates that the window has been destroyed and you
    /// should exit your event loop and no longer perform operations on the
    /// window
    Quit,
    MouseMove { x: f32, y: f32 },
    MouseButton {
        button: MouseButton,
        state: KeyState,
        x: f32,
        y: f32,
    },
}

#[derive(Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}
