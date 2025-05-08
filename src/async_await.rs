// async fn returns a Future that, when awaited, yields a u32
async fn fetch_value() -> u32 {
    42
}

// attribute sets up tokio executor & enables main-like function
#[tokio::main]
pub async fn asynchronous() {
    // call async fn & suspend until it returns
    let v = fetch_value().await;
    println!("got {}", v);
}
