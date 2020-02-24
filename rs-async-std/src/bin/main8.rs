// https://docs.rs/async-std/1.5.0/async_std/future/fn.timeout.html

use async_std;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let future = async_std::task::sleep(std::time::Duration::from_secs(1));
    // &std::future::GenFuture<async_std::task::sleep::sleep::{{closure}}>
    println!("{}", type_of(&future));

    let race = async_std::future::timeout(std::time::Duration::from_millis(500), future);

    if let Err(e) = async_std::task::block_on(race) {
        // TimeoutError { _private: () }
        println!("did not receive value within 500 ms. {:?}", e);
    }
}
