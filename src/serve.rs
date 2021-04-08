use crate::{Config, State};
use async_std::fs::{remove_file, OpenOptions};
use async_std::io;
use async_std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use uuid::Uuid;

async fn put(req: Request<Arc<State>>) -> tide::Result<String> {
    let name = Uuid::new_v4().to_string();
    let path = req.state().dir_path.join(&name);
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .await?;
    io::copy(req, file).await?;
    Ok(name)
}

async fn get(req: Request<Arc<State>>) -> tide::Result<Response> {
    let path = req.state().dir_path.join(req.param("file")?);
    match Body::from_file(path).await {
        Ok(body) => Ok(body.into()),
        Err(_) => Ok(Response::new(StatusCode::NotFound)),
    }
}

async fn delete(req: Request<Arc<State>>) -> tide::Result<Response> {
    let path = req.state().dir_path.join(req.param("file")?);
    remove_file(&path).await?;
    Ok(Response::new(StatusCode::Ok))
}

pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let srv_addr = config.srv_addr.clone();
    let mut app = tide::with_state(Arc::new(State::from_config(config).await?));
    app.at("/").put(put);
    app.at(":file").put(put).get(get).delete(delete);
    app.listen(srv_addr).await?;
    Ok(())
}
