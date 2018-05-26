#![cfg(windows)]

#[macro_use]
extern crate lazy_static;

extern crate direct2d;
extern crate direct3d11;
extern crate directwrite;
extern crate dxgi;
extern crate uuid;
extern crate winapi;
extern crate windows_dpi;

mod internals;

pub mod builder;
pub mod error;
pub mod event;
pub mod window;
