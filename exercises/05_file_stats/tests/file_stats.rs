use std::path::PathBuf;

use stage05_file_stats::analyze_file;

#[test]
fn analyzes_a_real_file() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample.txt");

    let stats = analyze_file(path).expect("fixture should be readable");
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.words, 6);
}

#[test]
fn reports_missing_file() {
    let error = analyze_file("a-file-that-does-not-exist.txt")
        .expect_err("missing file should return an error");
    assert!(error.to_string().contains("读取文件失败"));
}
