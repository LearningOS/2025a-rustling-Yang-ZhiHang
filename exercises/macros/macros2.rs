// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// Macro will fold in where the macro is invoked, so we need to define it
// before we can use it.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    hello();;
    my_macro!();
}