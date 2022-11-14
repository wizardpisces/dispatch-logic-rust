
use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

pub fn hello_macro() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
}