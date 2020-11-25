mod test_db;
mod test_helpers;

use assert_json_diff::assert_json_eq;
use sqlx::PgPool;
use surf;

use super::*;

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
async fn test_2() -> tide::Result<()> {
    Ok(())
}
