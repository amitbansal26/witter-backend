use sqlx::{PgPool, Pool, Postgres};
use std::env;

#[allow(dead_code)]
#[allow(unused)]
pub struct TestDatabase {
    db_url: String,
    db_pool: Option<Pool<Postgres>>,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let db_url = db_url();
        create_db(&db_url).await;
        //run_migrations(&db_url).await;

        let db_pool = PgPool::connect(&db_url).await.unwrap();

        Self {
            db_url,
            db_pool: Some(db_pool),
        }
    }

    pub fn db(&self) -> PgPool {
        self.db_pool.clone().unwrap()
    }
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
    let separator_pos = db_url.rfind('/').unwrap();
    let pg_conn = &db_url[..=separator_pos];
    let db_name = &db_url[separator_pos + 1..];
    (pg_conn, db_name)
}

async fn create_db(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);

    let conn = PgPool::connect(pg_conn).await.unwrap();
    let sql = format!(r#"CREATE DATABASE "{}""#, &db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn.acquire().await.unwrap())
        .await
        .unwrap();
}

#[allow(dead_code)]
async fn drop_db(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);
    let conn = PgPool::connect(pg_conn).await.unwrap();

    // Disconnect any existing connections to the DB
    let sql = format!(
        r#"SELECT pg_terminate_backend(pg_stat_activity.pid)
FROM pg_stat_activity
WHERE pg_stat_activity.datname = '{db}'
AND pid <> pg_backend_pid();"#,
        db = db_name
    );
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn.acquire().await.unwrap())
        .await
        .unwrap();

    let sql = format!(r#"DROP DATABASE "{db}";"#, db = db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn.acquire().await.unwrap())
        .await
        .unwrap();
}

#[allow(dead_code)]
async fn run_migrations(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);
    let conn = PgPool::connect(&format!("{}/{}", pg_conn, db_name))
        .await
        .unwrap();

    // Run the migrations
    let sql = async_std::fs::read_to_string("../bin/backend/setup.sql")
        .await
        .unwrap();

    for query in sql.split(';') {
        sqlx::query::<Postgres>(&query)
            .execute(&mut conn.acquire().await.unwrap())
            .await
            .unwrap();
    }
}
