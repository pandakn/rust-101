use rust_101::http::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    if let Err(e) = server.run() {
        println!("{}", e)
    }
}
