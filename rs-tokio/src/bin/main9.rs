// https://docs.rs/tokio/0.2.11/tokio/runtime/struct.Runtime.html

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let mut event_loop = tokio::runtime::Runtime::new().unwrap();
    let task = event_loop.spawn(async {
        println!("{}", 1)
    });

    // core::result::Result<(), tokio::task::error::JoinError>
    let r : Result<_,_> = event_loop.block_on(task);
    println!("{}", type_of(r))
}