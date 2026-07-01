use tokio::sync::OnceCell;
use sqlx::MySqlPool;

static DB: OnceCell<MySqlPool> = OnceCell::const_new();
static DB_URL: &str = "mysql://root:secret_password@localhost:3306/my_database";

async fn init_db() -> MySqlPool {
    use sqlx::mysql::MySqlPoolOptions;

    MySqlPoolOptions::new() 
            .max_connections(10) 
            .connect(DB_URL)
            .await
            .unwrap()
}

pub async fn get_db() -> &'static MySqlPool {
    DB.get_or_init(init_db).await
}