// https://docs.rs/async-std/1.5.0/async_std/task/fn.block_on.html

use async_std::task;

async fn say_hello() -> () {
    println!("Hello, world!");
}

fn main() {
    task::block_on(say_hello());
}