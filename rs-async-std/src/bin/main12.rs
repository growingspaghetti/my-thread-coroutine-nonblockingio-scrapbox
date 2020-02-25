use async_std;

fn greet() -> impl std::future::Future<Output=String> {
    async {
        "HelloWorld".to_string()
    }
}

#[async_std::main]
async fn main() {
    // HelloWorld
    println!("{}", greet().await);
}
