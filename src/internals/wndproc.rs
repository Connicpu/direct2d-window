use internals::data::WindowInner;
use internals::dpi;
use event::{Event, KeyState, MouseButton};

use std::panic;
use std::panic::AssertUnwindSafe;

use user32;
use winapi::*;

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wp: WPARAM,
    lp: LPARAM,
) -> LRESULT {
    let lpwindow = user32::GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const WindowInner;
    let inner: Option<&WindowInner> = if !lpwindow.is_null() {
        Some(&*lpwindow)
    } else {
        None
    };

    let (x, y) = if let Some(inner) = inner {
        let scale = inner.dpi_scale.get();
        let (x, y) = (LOWORD(lp as DWORD) as f32, HIWORD(lp as DWORD) as f32);
        (x / scale, y / scale)
    } else {
        (0.0, 0.0)
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| match (inner, msg) {
        (_, WM_CREATE) => {
            let params = &*(lp as LPCREATESTRUCTW);
            let lpwindow = params.lpCreateParams as *const WindowInner;
            let window: &WindowInner = &*lpwindow;
            window.hwnd.set(hwnd);
            window.dpi_scale.set(dpi::get_dpi_for(hwnd));
            user32::SetWindowLongPtrW(hwnd, GWLP_USERDATA, lpwindow as LONG_PTR);
            0
        }
        (Some(inner), WM_CLOSE) => {
            inner.events.borrow_mut().push_back(Event::CloseRequest);
            0
        }
        (Some(inner), WM_DESTROY) => {
            inner.events.borrow_mut().push_back(Event::Quit);
            0
        }
        (Some(inner), WM_DPICHANGED) => {
            let new_dpi = (wp & 0xFFFF) as f32 / 96.0;
            inner.dpi_scale.set(new_dpi);

            let event = Event::DpiChanged { new_dpi };
            inner.events.borrow_mut().push_back(event);
            0
        }

        ///////////////////////////////////////////////
        // Mouse button down
        (Some(inner), WM_LBUTTONDOWN) => {
            let event = Event::MouseButton {
                button: MouseButton::Left,
                state: KeyState::Pressed,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_RBUTTONDOWN) => {
            let event = Event::MouseButton {
                button: MouseButton::Right,
                state: KeyState::Pressed,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_MBUTTONDOWN) => {
            let event = Event::MouseButton {
                button: MouseButton::Middle,
                state: KeyState::Pressed,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_XBUTTONDOWN) => {
            let event = Event::MouseButton {
                button: match HIWORD(lp as DWORD) & 0b11 {
                    1 => MouseButton::X1,
                    2 => MouseButton::X2,
                    _ => unreachable!(),
                },
                state: KeyState::Pressed,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }

        ///////////////////////////////////////////////
        // Mouse button up
        (Some(inner), WM_LBUTTONUP) => {
            let event = Event::MouseButton {
                button: MouseButton::Left,
                state: KeyState::Released,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_RBUTTONUP) => {
            let event = Event::MouseButton {
                button: MouseButton::Right,
                state: KeyState::Released,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_MBUTTONUP) => {
            let event = Event::MouseButton {
                button: MouseButton::Middle,
                state: KeyState::Released,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }
        (Some(inner), WM_XBUTTONUP) => {
            let event = Event::MouseButton {
                button: match HIWORD(lp as DWORD) & 0b11 {
                    1 => MouseButton::X1,
                    2 => MouseButton::X2,
                    _ => unreachable!(),
                },
                state: KeyState::Released,
                x,
                y,
            };
            inner.events.borrow_mut().push_back(event);
            0
        }

        //////////////////////////////////////////////////////
        // Other mouse stuff
        (Some(inner), WM_MOUSEMOVE) => {
            let event = Event::MouseMove { x, y };
            inner.events.borrow_mut().push_back(event);
            0
        }
        _ => user32::DefWindowProcW(hwnd, msg, wp, lp),
    }));

    // Panic handling
    match result {
        Ok(lres) => lres,
        Err(panic) => {
            let lpwindow = user32::GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const WindowInner;
            if !lpwindow.is_null() {
                let inner: &WindowInner = &*lpwindow;
                inner.panic.set(Some(panic));
            }
            user32::DefWindowProcW(hwnd, msg, wp, lp)
        }
    }
}
