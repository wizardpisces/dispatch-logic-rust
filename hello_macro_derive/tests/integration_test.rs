use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro()->String;
}

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

#[test]
pub fn hello_macro() {
    assert_eq!(Sunfei::hello_macro(),"Hello, Macro! My name is Sunfei!");
    assert_eq!(Sunface::hello_macro(),"Hello, Macro! My name is Sunface!");
}

