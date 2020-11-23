use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()>{
    dotenv::dotenv().ok();
    let db_url= std::env::var("DATABASE_URL").unwrap();
    dbg!(db_url);


    let mut app = tide::new();
    app.at("/hello").get(greeting_func);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
async fn greeting_func(mut req: Request<()>) -> tide::Result{
  Ok("Hello world".into())
}
