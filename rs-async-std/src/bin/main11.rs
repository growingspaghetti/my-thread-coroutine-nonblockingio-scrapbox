// https://doc.rust-lang.org/nightly/core/future/trait.Future.html
// https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future

use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin; // pointer pinning
use async_std;

struct Greeting {}

impl Future for Greeting {
    type Output = String;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready("Hello World".to_string()) // js: Promise.resolve(value)
        //Poll::Pending
    }
}

#[async_std::main]
async fn main() {
    // Hello World
    println!("{}", Greeting{}.await);
}
