use async_std;

struct MyError {}

fn main() {
    let future = async {
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        println!("1");
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Result::<&str, MyError>::Ok("2")
    };
    let task = async_std::task::spawn(future);
    println!("0");
    if let Ok(i) = async_std::task::block_on(task) {
        println!("{}", i);
    }
}
