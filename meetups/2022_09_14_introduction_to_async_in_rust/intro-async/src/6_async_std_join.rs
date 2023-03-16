use async_std;

#[async_std::main]
async fn main() {
    let (first, second) = futures::join!(
        do_stuff_async(),
        more_async_work());
    // do something with the values
}

async fn do_stuff_async() -> u32 {
    17
}

async fn more_async_work() -> u32 {
    257
}

async fn func3() -> u32 {
    65537
}