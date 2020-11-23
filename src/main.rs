use tide::Request;
use sqlx::{Pool, PgPool, Error, Postgres};

#[async_std::main]
async fn main() -> tide::Result<()>{
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_url).await?;
    let rows = sqlx::query!("select (1) as one").fetch_one(&db_pool).await?;
    dbg!(rows);
    let mut app = tide::Server::with_state(State { db_pool: db_pool });
    app.at("/hello").get(greeting_func);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(Clone, Debug)]
struct State {
    db_pool: Pool<Postgres>,
}
async fn  greeting_func(req: Request<State> ) -> tide::Result{
    let pool = &req.state().db_pool;
    println!("{}", pool.size());
  Ok("Hello world".into())
}
