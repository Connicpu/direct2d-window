use builder::styles::WindowClassStyle;
use builder::WindowProperties;
use error::DResult;
use internals::data::WindowInner;
use internals::wndproc::window_proc;
use window::Window;

use std::panic;
use std::sync::Arc;
use std::{fmt, mem, ptr};

use direct2d::Factory;
use uuid::Uuid;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::winerror::{FACILITY_WIN32, HRESULT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleExW;
use winapi::um::winuser::*;

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
            let res = GetModuleHandleExW(
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

    let atom = unsafe { RegisterClassExW(&classdef) };
    if atom == 0 {
        return Err(hresult_from_win32(unsafe { GetLastError() }));
    }

    Ok(WindowClass {
        reg: Arc::new(WindowClassReg { atom: atom }),
    })
}

fn get_title(title: &Option<String>) -> Vec<u16> {
    title
        .as_ref()
        .map(|s| &s[..])
        .unwrap_or("Untitled window")
        .encode_utf16()
        .chain(Some(0))
        .collect()
}

pub fn create_window(
    class: WindowClass,
    props: &WindowProperties,
    factory: Factory,
) -> DResult<Window> {
    let title: Vec<u16> = get_title(&props.title);

    let window_inner: Box<WindowInner> = Box::new(WindowInner::create(class.clone(), factory)?);
    unsafe {
        let hwnd = CreateWindowExW(
            props.style.style_flags_ex(),
            class.reg.atom as LPCWSTR,
            title.as_ptr(),
            props.style.style_flags(),
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            MODULE_HANDLE.as_hinst(),
            &*window_inner as *const _ as *mut _,
        );

        if hwnd == (!0) as *mut _ {
            Err(hresult_from_win32(GetLastError()))?;
        }

        assert_eq!(hwnd, window_inner.hwnd.get());

        ShowWindow(hwnd, SW_SHOW);
    }

    Ok(Window::new(window_inner))
}

pub fn peek_message(inner: &WindowInner) -> bool {
    let hwnd = inner.hwnd.get();
    unsafe {
        let mut msg = mem::uninitialized();
        let any = PeekMessageW(&mut msg, hwnd, 0, 0, PM_REMOVE) != 0;
        TranslateMessage(&msg);
        DispatchMessageW(&msg);

        if let Some(panic) = inner.panic.replace(None) {
            panic::resume_unwind(panic);
        }

        any
    }
}

pub fn wait_message(inner: &WindowInner) -> bool {
    let hwnd = inner.hwnd.get();
    unsafe {
        let mut msg = mem::uninitialized();
        let any = GetMessageW(&mut msg, hwnd, 0, 0) != 0;
        TranslateMessage(&msg);
        DispatchMessageW(&msg);

        if let Some(panic) = inner.panic.replace(None) {
            panic::resume_unwind(panic);
        }

        any
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
            UnregisterClassW(self.atom as LPCWSTR, MODULE_HANDLE.as_hinst());
        }
    }
}
