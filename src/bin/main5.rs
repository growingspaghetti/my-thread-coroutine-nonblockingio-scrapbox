use tokio::task;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() {
    // std::future::GenFuture<main5::main::{{closure}}::{{closure}}::{{closure}}>
    println!("{}", type_of((|| async {"Hello World"})()));
    // main5::main::{{closure}}::{{closure}}
    println!("{}", type_of(|| async {"Hello World"}));

    let handle: task::JoinHandle<_> = tokio::spawn((|| async {"Hello World"})());
    // Hello World
    println!("{}", handle.await.unwrap());
}