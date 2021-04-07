use crud_file_server::{serve, Config};

#[async_std::main]
async fn main() {
    let config = Config::from_env();
    println!("Serving at {}", config.srv_addr);
    serve(config).await.unwrap();
}
