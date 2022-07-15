use redis::aio::ConnectionManager;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct Store {
    pub redis_con: ConnectionManager,
    pub pg_pool: PgPool
}

impl Store {
   pub async fn database_setup(psql_url: String, redis_url: String) -> Store {
        let redis_client = redis::Client::open(redis_url).expect("Couldn't connect to the REDIS database!");
        let pg_pool = PgPoolOptions::new().max_connections(1).connect(&psql_url).await.expect("Couldn't connect to the POSTGRES database!");
        let redis_con = ConnectionManager::new(redis_client.clone()).await.unwrap();

        Store { redis_con, pg_pool }
   } 
}
