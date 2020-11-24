use super::*;
use sqlx::{PgPool};
use assert_json_diff::assert_json_eq;
use surf;

#[async_std::test]
async fn test_1() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_url).await.unwrap();
    let server = create_server(db_pool).await;
    let mut res = surf::Client::with_http_client(server).get("http://localhost:8080/hello").await?;
//    let json = res.body_json().await?;
    let resstr = res.body_string().await?;
    assert_eq!("[1,2,3]", resstr);
    Ok(())
}
