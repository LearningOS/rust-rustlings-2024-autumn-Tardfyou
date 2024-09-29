// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


enum MyEnum {
    SomeValue(String),
    AnotherValue(u32),
}

fn main() {
    let my_var = MyEnum::SomeValue(String::from("Hello, Rust"));

    match my_var {
        MyEnum::SomeValue(s) => {
            println!("Got ownership of string: {}", s);
            // 在此分支中，我们获取了字符串的所有权，可以自由地使用它
        }
        MyEnum::AnotherValue(n) => {
            println!("Got ownership of u32: {}", n);
            // 在此分支中，我们获取了 u32 的所有权，可以自由地使用它
        }
    }

    // 注意，在 `match` 表达式之后，`my_var` 的所有权并未返回，因为在所有可能的分支中都已经被取走。
}


