use async_std::path::PathBuf;
use async_std::{fs, task};
use crud_file_server::{serve, Config};

#[async_std::test]
async fn crud() {
    let config = Config {
        dir_path: PathBuf::from("temp_content"),
        srv_addr: "0.0.0.0:8000".into(),
    };
    fs::remove_dir_all(&config.dir_path).await.ok();
    let serve_handle = {
        let config = config.clone();
        task::spawn(async move {
            serve(config.clone()).await.unwrap();
        })
    };
    task::sleep(std::time::Duration::from_secs(1)).await;

    assert_eq!(
        surf::get(format!("http://{}/none", config.srv_addr))
            .await
            .unwrap()
            .status(),
        404
    );

    let payload = "";
    let mut res = surf::post(format!("http://{}", config.srv_addr))
        .body(payload)
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let uuid = res.body_string().await.unwrap();

    let mut res = surf::get(format!("http://{}/{}", config.srv_addr, uuid))
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.body_bytes().await.ok(),
        Some(payload.as_bytes().to_vec())
    );

    let payload = "Hello, World!";
    let res = surf::put(format!("http://{}/{}", config.srv_addr, uuid))
        .body(payload)
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let mut res = surf::get(format!("http://{}/{}", config.srv_addr, uuid))
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.body_bytes().await.ok(),
        Some(payload.as_bytes().to_vec())
    );

    assert_eq!(
        surf::delete(format!("http://{}/{}", config.srv_addr, uuid))
            .await
            .unwrap()
            .status(),
        200
    );

    assert_eq!(
        surf::get(format!("http://{}/{}", config.srv_addr, uuid))
            .await
            .unwrap()
            .status(),
        404
    );

    serve_handle.cancel().await;
    fs::remove_dir_all(&config.dir_path).await.ok();
}
