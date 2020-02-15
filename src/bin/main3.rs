use tokio::task;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() {
    // Note that it's not std::Future but opaque impl Future
    // &std::future::GenFuture<main3::main::{{closure}}::{{closure}}>
    let future_func = async {"Hello World"};
    println!("{}", type_of(&future_func));

    let handle: task::JoinHandle<_> = tokio::spawn(future_func);
    // Hello World
    println!("{}", handle.await.unwrap());
}