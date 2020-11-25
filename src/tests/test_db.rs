use rand::Rng;
use sqlx::{PgConnection, Pool, Postgres};
use std::env;

pub struct TestDatabase {
    db_url: String,
    db_pool: Option<Pool<Postgres>>,
}

fn db_url() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let rng = thread_rng();
    let suffix: String = rng.sample_iter(&Alphanumeric).take(16).collect();
    let db_url = env::var("DATABASE_URL").expect("Database url not found");
    format!("{}_{}", db_url, suffix)
}

fn parse_db_url(db_url: &str) -> (&str, &str) {
    // Create the DB, splitting the url on the last slash
    // postgres://localhost/legasea_test_aoeuaoeu
    let separator_pos = db_url.rfind("/").unwrap();
    let pg_conn = &db_url[..=separator_pos];
    let db_name = &db_url[separator_pos + 1..];
    (pg_conn, db_name)
}

async fn create_db(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);

    let mut conn = sqlx::postgres::PgConnection::connect(pg_conn)
        .await
        .unwrap();

    let sql = format!(r#"CREATE DATABASE "{}""#, &db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}
