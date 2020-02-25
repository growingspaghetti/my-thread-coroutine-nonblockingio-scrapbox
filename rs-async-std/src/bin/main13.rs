// https://docs.rs/futures-util/0.3.4/src/futures_util/future/future/map.rs.html#33-49
// https://medium.com/dwelo-r-d/my-experience-porting-old-rust-futures-to-async-await-744430e9c576

use async_std;

fn greet() -> impl std::future::Future<Output=Result<String,()>> {
    async {
        Ok("HelloWorld".to_string())
    }
}

async fn greet_then_reverse() -> Result<String, ()> {
    Ok(greet().await?.chars().rev().collect::<String>())
}

#[async_std::main]
async fn main() {
    // error. not map function but use await? instead.
    // println!("{}", greet().map(|v| v.chars().rev().collect::<String>()).await);
    // Ok("dlroWolleH")
    println!("{:?}", greet_then_reverse().await);
}
