use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub frequencies: HashMap<String, usize>,
}

impl TextStats {
    pub fn most_common_word(&self) -> Option<(&str, usize)> {
        self.frequencies
            .iter()
            .map(|(word, count)| (word.as_str(), *count))
            .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(left.0)))
    }
}

pub trait Report {
    fn report(&self) -> String;
}

impl Report for TextStats {
    fn report(&self) -> String {
        let most_common = self
            .most_common_word()
            .map(|(word, count)| format!("{word} ({count})"))
            .unwrap_or_else(|| String::from("无"));

        format!(
            "行数：{}\n单词数：{}\n字节数：{}\n最高频：{}",
            self.lines, self.words, self.bytes, most_common
        )
    }
}

pub fn analyze(text: &str) -> TextStats {
    let mut frequencies = HashMap::new();
    let mut words = 0;

    for word in text.split_whitespace().map(str::to_lowercase) {
        words += 1;
        *frequencies.entry(word).or_insert(0) += 1;
    }

    TextStats {
        lines: if text.is_empty() {
            0
        } else {
            text.lines().count()
        },
        words,
        bytes: text.len(),
        frequencies,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_multiple_lines() {
        let stats = analyze("Rust is fast\nRust is safe");

        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 6);
        assert_eq!(stats.frequencies.get("rust"), Some(&2));
        assert_eq!(stats.most_common_word(), Some(("is", 2)));
    }

    #[test]
    fn handles_empty_text() {
        let stats = analyze("");
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.most_common_word(), None);
    }

    #[test]
    fn creates_a_report() {
        let report = analyze("rust rust").report();
        assert!(report.contains("单词数：2"));
        assert!(report.contains("rust (2)"));
    }
}
