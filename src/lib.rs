#[macro_use]
extern crate lazy_static;

extern crate direct2d;
extern crate kernel32;
extern crate libloading;
extern crate user32;
extern crate gdi32;
extern crate uuid;
extern crate winapi;

mod internals;

pub mod builder;
pub mod event;
pub mod window;
