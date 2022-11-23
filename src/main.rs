mod constant;
mod text;
mod practice;

// use text::dispatch_logic::portal;
// use hello_macro::hello_macro;
use practice::{args_parse,guess_game};

fn main() {
    // hello_macro(); // test macro
   
    // args_parse::start();
    guess_game::start();
    
}
