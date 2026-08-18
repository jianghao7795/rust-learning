use stage08_todo_api::{AppState, router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("待办 API 已启动：http://{address}");
    axum::serve(listener, router(AppState::default())).await?;
    Ok(())
}
