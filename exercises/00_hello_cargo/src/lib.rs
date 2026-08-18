/// Builds the greeting printed by the command-line program.
pub fn greeting(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        String::from("你好，Rust！")
    } else {
        format!("你好，{name}！欢迎学习 Rust。")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_a_named_learner() {
        assert_eq!(greeting("小明"), "你好，小明！欢迎学习 Rust。");
    }

    #[test]
    fn uses_a_default_for_blank_input() {
        assert_eq!(greeting("  "), "你好，Rust！");
    }
}
