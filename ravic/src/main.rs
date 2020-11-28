extern crate lexer;
fn main() {
    let input = "hello";
    let mut source = lexer::source::Source::new(input);
    println!("First {} second {}!", source.getc(), source.getc());
}
