use async_scoped;;

#[tokio::main]
async fn main() {
    let str = "Hello world";
    tokio_scoped::scope(|s| {
      s.spawn(|| {
        println!("str = {}", str);
      });
    });
}