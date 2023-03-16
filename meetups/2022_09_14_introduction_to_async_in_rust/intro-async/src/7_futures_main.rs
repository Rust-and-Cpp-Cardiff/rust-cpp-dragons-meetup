use futures::executor; // 0.3.1

fn main() {
    let v = executor::block_on(some_async_fn());
    println!("{}", v);
}

async fn some_async_fn() -> u32 {
    42
}