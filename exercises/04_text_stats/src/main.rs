use stage04_text_stats::{Report, analyze};

fn main() {
    let text = "Rust is fast.\nRust is memory safe.";
    println!("{}", analyze(text).report());
}
