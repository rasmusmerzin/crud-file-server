use async_std::fs::{remove_file, OpenOptions};
use async_std::io;
use async_std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use uuid::Uuid;

mod config;
mod state;

use config::Config;
use state::State;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();
    let srv_addr = config.srv_addr.clone();
    let mut app = tide::with_state(Arc::new(State::from_config(config).await?));

    app.at(":file")
        .put(|req: Request<Arc<State>>| async move {
            let name = Uuid::new_v4().to_string();
            let path = req.state().dir_path.join(&name);

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .await?;

            io::copy(req, file).await?;

            Ok(name)
        })
        .get(|req: Request<Arc<State>>| async move {
            let path = req.state().dir_path.join(req.param("file")?);

            match Body::from_file(path).await {
                Ok(body) => Ok(body.into()),
                Err(_) => Ok(Response::new(StatusCode::NotFound)),
            }
        })
        .delete(|req: Request<Arc<State>>| async move {
            let path = req.state().dir_path.join(req.param("file")?);

            remove_file(&path).await?;

            Ok(Response::new(StatusCode::Ok))
        });

    app.listen(srv_addr).await?;
    Ok(())
}
