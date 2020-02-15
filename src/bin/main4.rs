use tokio::task;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() {
    // main4::main::{{closure}}::{{closure}}
    println!("{}", type_of(|| "Hello World"));
    // &str
    println!("{}", type_of((|| "Hello World")()));

    // calling closure in async manner
    let hello_world = || "Hello World";
    let handle: task::JoinHandle<_> = tokio::spawn(async move { hello_world });
    // Hello World
    println!("{}", handle.await.unwrap()());
}