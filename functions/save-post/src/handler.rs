use std::env;

use lambda_http::{Error, IntoResponse, Request, RequestExt, Response, Body};
use sqlx::MySqlPool;
use ytakada_dev_adapter::persistence::post::{MysqlPostRepository, PostRepository};

use crate::service;

pub async fn handler(req: Request) -> Result<impl IntoResponse, Error> {
    let post_id = req
        .path_parameters()
        .first("id")
        .expect("Post id should be provided.")
        .to_string();

    let body = match req.body() {
        Body::Text(text) => Ok(text),
        _ => Err(()),
    }.unwrap();

    let repository = MysqlPostRepository {
        pool: MySqlPool::connect("mysql://root:password@mysql/local").await.unwrap(),
    };

    let post = service::save_post(post_id, body, &repository).await.unwrap();

    let res = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
        .header("Access-Control-Allow-Credential", "true")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&post).unwrap())
        .expect("failed to render response");

    Ok(res)
}
