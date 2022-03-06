use tokio::net::{TcpStream, TcpListener};
use mini_redis::{Connection, Frame};
// use

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:>}", frame);

        let response = Frame::Error("unimplemented".into());
        connection.write_frame(&response).await.unwrap();
    }
}
