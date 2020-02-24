// https://docs.rs/tokio/0.2.11/tokio/task/struct.JoinHandle.html
use tokio::task;

async fn say_hello() -> () {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    let handle: task::JoinHandle<_> = tokio::spawn(say_hello());
    // Ok()
    println!("{:?}", handle.await);
}