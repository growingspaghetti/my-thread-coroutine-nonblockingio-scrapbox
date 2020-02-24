// https://docs.rs/tokio/0.2.11/tokio/time/fn.timeout.html
// https://docs.rs/tokio/0.2.11/src/tokio/time/delay.rs.html#81-99 impl Future for Delay

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[tokio::main]
async fn main() {
    // tokio::time::delay::Delay
    println!("{}", type_of(tokio::time::delay_for(std::time::Duration::from_secs(1))));

    // let future = tokio::time::delay_for(std::time::Duration::from_secs(1)) is fine though
    let future = async {
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await; // I mean lengthy calc block
    };

    if let Err(e) = tokio::time::timeout(std::time::Duration::from_millis(500), future).await {
        // Elapsed(())
        println!("did not receive value within 500 ms. {:?}", e);
    }
}
