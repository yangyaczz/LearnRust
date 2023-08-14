macro_rules! greeting {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greeting!("TinTinland Rust");
}
