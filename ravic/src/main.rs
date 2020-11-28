extern crate lexer;
fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, lexer::add_one(num));
}
