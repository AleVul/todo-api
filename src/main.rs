use api_todo_io::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088").expect("Failed to bind random port");
    run(listener)?.await
}
