use async_std;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

async fn six() -> i32 {
    let one = async {1};
    let two = async {2};
    let three = async {3};

    let task_one = async_std::task::spawn(one);
    // &async_std::task::join_handle::JoinHandle<i32>
    println!("{}", type_of(&task_one));

    let task_two = async_std::task::spawn(two);
    let task_three = async_std::task::spawn(three);

    task_one.await + task_two.await + task_three.await
}

fn main() {
    // 6
    println!("{}", async_std::task::block_on(six()));
}
