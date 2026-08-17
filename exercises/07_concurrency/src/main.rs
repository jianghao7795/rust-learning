use stage07_concurrency::square_all;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values = vec![9, 3, 7, 2, 5, 8];
    let results = square_all(&values, 3)?;
    println!("输入：{values:?}");
    println!("平方：{results:?}");
    Ok(())
}
