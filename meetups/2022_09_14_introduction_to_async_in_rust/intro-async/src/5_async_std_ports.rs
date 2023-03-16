use async_std::{
    net::{TcpListener, TcpStream}, // 3
};

#[async_std::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // A new task is spawned for each inbound socket.  The socket is
        // moved to the new task and processed there.
        async_std::task::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(_socket: TcpStream) {
    println!("Processing connection");
}
