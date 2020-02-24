use async_std::task;
fn main() {
    task::spawn(async {
        println!("foo");
    });
    task::spawn(async {
        println!("bar");
    });
    task::spawn(async {
        println!("baz");
    });
    // thread 'async-std/executor' panicked at 
    task::block_on(async {}); // pause.
}