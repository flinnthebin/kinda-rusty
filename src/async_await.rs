async fn fetch_value() -> u32 {
    42
}

#[tokio::main]
pub async fn asynchronous() {
    let v = fetch_value().await;
    println!("got {}", v);
}
