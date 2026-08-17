use stage02_ownership::{append_world, first_word, largest, longer};

fn main() {
    let mut greeting = String::from("hello");
    append_world(&mut greeting);

    println!("完整文本：{greeting}");
    println!("第一个单词：{}", first_word(&greeting));
    println!("较长文本：{}", longer(&greeting, "rust"));
    println!("最大数字：{:?}", largest(&[5, 2, 8, 1]));
}
