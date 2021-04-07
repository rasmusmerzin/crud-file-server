use crud_file_server::{serve, Config};

#[async_std::main]
async fn main() {
    serve(Config::from_env()).await.unwrap();
}
