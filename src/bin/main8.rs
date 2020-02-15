// https://docs.rs/tokio/0.2.11/tokio/time/fn.timeout.html

#[tokio::main]
async fn main() {
    let future = async {
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    };

    if let Err(e) = tokio::time::timeout(std::time::Duration::from_millis(500), future).await {
        // Elapsed(())
        println!("did not receive value within 500 ms. {:?}", e);
    }
}