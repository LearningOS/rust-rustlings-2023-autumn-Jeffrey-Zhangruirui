// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("This is the first macro!");
    }
    
}

fn main() {
    my_macro!();      // 调用第一个宏规则，没有参数
 
}

