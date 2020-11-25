use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};
use tide::{Request, Response};

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_url).await?;
    let rows = sqlx::query!("select (1) as one")
        .fetch_one(&db_pool)
        .await?;
    dbg!(rows);
    let server = create_server(db_pool).await;
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn create_server(db_pool: Pool<Postgres>) -> tide::Server<State> {
    let mut app = tide::Server::with_state(State { db_pool: db_pool });
    app.at("/hello").get(greeting_func);
    app
}

#[derive(Clone, Debug)]
struct State {
    db_pool: Pool<Postgres>,
}

async fn greeting_func(req: Request<State>) -> tide::Result {
    let _pool = &req.state().db_pool;
    let json = json!([1, 2, 3]);
    Ok(Response::from(json))
}

#[cfg(test)]
mod tests;
