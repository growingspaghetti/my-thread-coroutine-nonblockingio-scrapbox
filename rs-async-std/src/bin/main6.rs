use async_std;

// lazy evaluation?
async fn six() -> i32 {
    let one = async {1};
    let two = async {2 + async_std::task::spawn(one).await};
    let three = async {3 + async_std::task::spawn(two).await};
    async_std::task::spawn(three).await
}

fn main() {
    // 6
    println!("{}", async_std::task::block_on(six()));
}
