use internals::data::WindowInner;

use std::panic;

use user32;
use winapi::*;

pub unsafe extern "system" fn window_proc(hwnd: HWND,
                                          msg: UINT,
                                          wp: WPARAM,
                                          lp: LPARAM)
                                          -> LRESULT {
    let result = panic::catch_unwind(|| match msg {
        WM_CREATE => {
            let params = &*(lp as LPCREATESTRUCTW);
            let lpwindow = params.lpCreateParams as *const WindowInner;
            let window: &WindowInner = &*lpwindow;
            window.hwnd.set(hwnd);
            user32::SetWindowLongPtrW(hwnd, GWLP_USERDATA, lpwindow as LONG_PTR);
            0
        }
        _ => user32::DefWindowProcW(hwnd, msg, wp, lp),
    });

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
