use async_std::task;
fn main() {
    let one = task::block_on(async {1});
    println!("{}", one); // 1
}