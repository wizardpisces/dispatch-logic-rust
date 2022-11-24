#![allow(unused_must_use)]
#[macro_export]
macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn write_html() {
        use std::fmt::Write;
        let mut out = String::new();

        write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are the best!"]]
        ]);

        print!("{}", out);
        assert_eq!(
            out,
            "<html><head><title>Macros guide</title></head>\
         <body><h1>Macros are the best!</h1></body></html>"
        );
    }
}
