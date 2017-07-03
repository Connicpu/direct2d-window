use libloading::{Library, Symbol};
use winapi::{BOOL, HRESULT, DWORD};

type TSetProcDpi = unsafe extern "system" fn(DWORD) -> HRESULT;
type TEnableProcDpi = unsafe extern "system" fn() -> BOOL;

lazy_static! {
    // This library may not exist on all supported playforms
    static ref SHCORE: Option<Library> = {
        Library::new("ShCore.dll").ok()
    };

    // This library always exists on supported platforms
    static ref USER32: Library = {
        Library::new("User32.dll").unwrap()
    };

    // This function may not exist on all supported playforms
    static ref SET_PROCESS_DPI_AWARENESS: Option<Symbol<'static, TSetProcDpi>> = {
        unsafe {
            SHCORE.as_ref().and_then(|shcore| shcore.get(b"SetProcessDpiAwareness\0").ok())
        }
    };

    // This function always exists on supported platforms, it's just not defined
    // by winapi at the moment
    static ref SET_PROCESS_DPI_AWARE: Symbol<'static, TEnableProcDpi> = {
        unsafe {
            USER32.get(b"SetProcessDPIAware\0").unwrap()
        }
    };
}

pub fn enable_dpi() {
    if let Some(set_awareness) = (*SET_PROCESS_DPI_AWARENESS).as_ref() {
        unsafe {
            set_awareness(1);
        }
    } else {
        unsafe {
            SET_PROCESS_DPI_AWARE();
        }
    }
}
