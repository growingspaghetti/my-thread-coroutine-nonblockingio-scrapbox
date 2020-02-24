// https://docs.rs/async-std/1.5.0/async_std/future/fn.poll_fn.html

use async_std;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn poll_greeting(_: &mut async_std::task::Context<'_>) -> async_std::task::Poll<String> {
    async_std::task::Poll::Ready("hello world".to_string())
}

fn main() {
    let future = async_std::future::poll_fn(poll_greeting);
    // std::future::GenFuture<async_std::future::poll_fn::poll_fn<main9::poll_greeting, alloc::string::String>::{{closure}}>
    println!("{}", type_of(&future));
    let greeting = async_std::task::block_on(future);
    // hello world
    println!("{}", greeting);
}
