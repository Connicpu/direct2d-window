use std::fmt;

use direct2d;
use direct3d11;
use directwrite;
use dxgi;

pub type DResult<T> = Result<T, Error>;

#[derive(Copy, Clone, PartialEq)]
pub enum Error {
    Dxgi(i32),
    D3D11(i32),
    D2D1(i32),
    DWrite(i32),
    Other(i32),
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (mut fmt, e) = match *self {
            Error::Dxgi(e) => (fmt.debug_tuple("Dxgi"), e),
            Error::D3D11(e) => (fmt.debug_tuple("D3D11"), e),
            Error::D2D1(e) => (fmt.debug_tuple("D2D1"), e),
            Error::DWrite(e) => (fmt.debug_tuple("DWrite"), e),
            Error::Other(e) => (fmt.debug_tuple("ComError"), e),
        };

        fmt.field(&e).field(&dxgi::Error(e).get_message()).finish()
    }
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::Dxgi(e) => e,
            Error::D3D11(e) => e,
            Error::D2D1(e) => e,
            Error::DWrite(e) => e,
            Error::Other(e) => e,
        }
    }
}

impl From<dxgi::Error> for Error {
    fn from(e: dxgi::Error) -> Error {
        Error::Dxgi(e.0)
    }
}

impl From<direct3d11::error::Error> for Error {
    fn from(e: direct3d11::error::Error) -> Error {
        match e {
            direct3d11::error::Error::D3D11(e) => Error::D3D11(e),
            direct3d11::error::Error::Dxgi(e) => e.into(),
        }
    }
}

impl From<direct2d::Error> for Error {
    fn from(e: direct2d::Error) -> Error {
        match e {
            direct2d::Error::ComError(e) => Error::D2D1(e),
            direct2d::Error::Dxgi(e) => e.into(),
        }
    }
}

impl From<directwrite::error::DWriteError> for Error {
    fn from(e: directwrite::error::DWriteError) -> Error {
        Error::DWrite(e.0)
    }
}

impl From<i32> for Error {
    fn from(e: i32) -> Error {
        Error::Other(e)
    }
}
