mod test_db;
mod test_helpers;

use sqlx::PgPool;

use super::*;
use crate::tests::test_helpers::{get, test_setup};
use serde_json::Value;

#[async_std::test]
async fn test_1() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_url).await.unwrap();
    let server = create_server(db_pool).await;
    let mut res = surf::Client::with_http_client(server)
        .get("http://localhost:8080/hello")
        .await?;
    let json: Vec<i32> = res.body_json().await?;
    //let resstr = res.body_string().await?;
    assert_eq!(vec![1, 2, 3], json);
    Ok(())
}

#[async_std::test]
#[allow(unused_must_use)]
async fn test_2() -> tide::Result<()> {
    let server = test_setup().await;
    let res = get("/hello").send(&server).await;
    let json: Value = res.0;
    assert_json_diff::assert_json_eq!(&json, &json!([1, 2, 3]));
    //println!("{:?}", res.0.to_string());
    Ok(())
}
