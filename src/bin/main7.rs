#[tokio::main]
async fn main() {
    let future = async {
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        println!("1");
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        "2"
    };
    let task = tokio::spawn(future);
    println!("0");
    if let Ok(i) = task.await {
        println!("{}", i);
    }
}