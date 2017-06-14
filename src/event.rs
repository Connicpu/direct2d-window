/// Window events for things that happen to your direct2d window.
/// All pixel values are measured in DIPs, which means their values
/// directly translate into the coordinates for direct2d. You can
/// get the current DPI scale from the Window if you need to convert
/// back to pixel coordinates.
pub enum Event {
    MouseMove { x: f32, y: f32 },
    MouseButton {
        button: MouseButton,
        state: KeyState,
        x: f32,
        y: f32,
    },
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

pub enum KeyState {
    Pressed,
    Released,
}
