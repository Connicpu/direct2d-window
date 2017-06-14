use internals::data::WindowInner;
use internals::wndproc::window_proc;
use builder::styles::WindowClassStyle;

use std::{mem, ptr, fmt, str};
use std::sync::Arc;

use kernel32;
use user32;
use uuid::Uuid;
use winapi::*;

#[derive(Copy, Clone, Debug)]
pub struct HModule(pub HMODULE);
unsafe impl Sync for HModule {}
impl HModule {
    pub fn as_hinst(self) -> HINSTANCE {
        self.0 as HINSTANCE
    }
}

lazy_static! {
    pub static ref MODULE_HANDLE: HModule = {
        unsafe {
            let mut module = ptr::null_mut();
            let res = kernel32::GetModuleHandleExW(
                // GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS |
                // GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                0x4 | 0x2,
                create_window_class as LPCWSTR,
                &mut module
            );
            assert!(res != 0);
            HModule(module)
        }
    };
}

#[inline]
pub fn hresult_from_win32(x: DWORD) -> HRESULT {
    if x <= 0 {
        x as HRESULT
    } else {
        ((x & 0x0000FFFF) | ((FACILITY_WIN32 as u32) << 16) | 0x80000000) as HRESULT
    }
}

fn generate_class_name() -> Vec<u16> {
    Uuid::new_v4()
        .hyphenated()
        .to_string()
        .chars()
        .map(|c| c as u32 as u16)
        .chain(Some(0))
        .collect::<Vec<_>>()
}

pub fn create_window_class(flags: WindowClassStyle) -> Result<WindowClass, HRESULT> {
    let classname = generate_class_name();

    let classdef = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: flags.style_flags(),
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: MODULE_HANDLE.as_hinst(),
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null_mut(),
        lpszClassName: classname.as_ptr(),
        hIconSm: ptr::null_mut(),
    };

    let atom = unsafe { user32::RegisterClassExW(&classdef) };
    if atom == 0 {
        return Err(hresult_from_win32(unsafe { kernel32::GetLastError() }));
    }

    Ok(WindowClass { reg: Arc::new(WindowClassReg { atom: atom }) })
}

pub fn create_window(class: WindowClass, flags: WindowStyle) -> Result<Window, HRESULT> {}

pub fn process_message(inner: &WindowInner) {
    let hwnd = inner.hwnd.get();
    unsafe {
        let mut msg = mem::uninitialized();
        user32::PeekMessageW(&mut msg, hwnd, 0, 0, PM_REMOVE);
        user32::TranslateMessage(&msg);
        user32::DispatchMessageW(&msg);
    }
}

#[derive(Clone)]
pub struct WindowClass {
    reg: Arc<WindowClassReg>,
}

impl fmt::Debug for WindowClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("WindowClass")
            .field("atom", &format_args!("0x{:x}", self.reg.atom))
            .field("refcount", &Arc::strong_count(&self.reg))
            .finish()
    }
}

struct WindowClassReg {
    atom: ATOM,
}

impl Drop for WindowClassReg {
    fn drop(&mut self) {
        unsafe {
            user32::UnregisterClassW(self.atom as LPCWSTR, MODULE_HANDLE.as_hinst());
        }
    }
}
