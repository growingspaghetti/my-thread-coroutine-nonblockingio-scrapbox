// https://docs.rs/tokio/0.2.11/tokio/runtime/struct.Runtime.html
fn main() {
    let mut event_loop = tokio::runtime::Runtime::new().unwrap();
    event_loop.block_on(async {
        println!("{}", 1)
    });
}