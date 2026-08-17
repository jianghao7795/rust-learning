use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: Level,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingTimestamp,
    MissingLevel,
    MissingMessage,
    UnknownLevel(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTimestamp => write!(formatter, "缺少时间"),
            Self::MissingLevel => write!(formatter, "缺少日志级别"),
            Self::MissingMessage => write!(formatter, "缺少消息"),
            Self::UnknownLevel(level) => write!(formatter, "未知日志级别：{level}"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LogSummary {
    pub valid_lines: usize,
    pub invalid_lines: usize,
    pub level_counts: HashMap<Level, usize>,
    pub errors: Vec<String>,
}

pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let mut parts = line.splitn(3, char::is_whitespace);
    let timestamp = parts.next().filter(|part| !part.is_empty());
    let level = parts.next().filter(|part| !part.is_empty());
    let message = parts.next().map(str::trim).filter(|part| !part.is_empty());

    let timestamp = timestamp.ok_or(ParseError::MissingTimestamp)?;
    let level = match level.ok_or(ParseError::MissingLevel)? {
        "DEBUG" => Level::Debug,
        "INFO" => Level::Info,
        "WARN" => Level::Warn,
        "ERROR" => Level::Error,
        other => return Err(ParseError::UnknownLevel(other.to_string())),
    };
    let message = message.ok_or(ParseError::MissingMessage)?;

    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        message: message.to_string(),
    })
}

pub fn analyze(text: &str) -> LogSummary {
    let mut summary = LogSummary::default();

    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        match parse_line(line) {
            Ok(entry) => {
                summary.valid_lines += 1;
                *summary.level_counts.entry(entry.level).or_insert(0) += 1;
                if entry.level == Level::Error {
                    summary.errors.push(entry.message);
                }
            }
            Err(_) => summary.invalid_lines += 1,
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_log_line() {
        let entry =
            parse_line("2026-08-17T10:00:00Z INFO server started").expect("line should be valid");
        assert_eq!(entry.level, Level::Info);
        assert_eq!(entry.message, "server started");
    }

    #[test]
    fn rejects_unknown_level() {
        assert_eq!(
            parse_line("2026-08-17 NOTICE hello"),
            Err(ParseError::UnknownLevel(String::from("NOTICE")))
        );
    }

    #[test]
    fn summarizes_valid_and_invalid_lines() {
        let summary = analyze(
            "2026-08-17T10:00:00Z INFO started\n\
             invalid\n\
             2026-08-17T10:01:00Z ERROR database unavailable",
        );

        assert_eq!(summary.valid_lines, 2);
        assert_eq!(summary.invalid_lines, 1);
        assert_eq!(summary.level_counts.get(&Level::Error), Some(&1));
        assert_eq!(summary.errors, vec!["database unavailable"]);
    }
}
