use mini_redis::client;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    println!("Setting in hello:world");
    let set_hello = client.set("hello", "world".into());

    // Set the key "hi" with value "rust and c++ cardiff"
    let mut client2 = client::connect("127.0.0.1:6379").await?;
    println!("Setting in hi:rust and c++ cardiff");
    let set_hi = client2.set("hi", "rust and c++ cardiff".into());

    // Wait for setting to complete
    tokio::try_join!(set_hello, set_hi)?;

    // Get key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    let result = client2.get("hi").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
