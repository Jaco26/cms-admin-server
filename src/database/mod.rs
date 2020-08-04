use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
  pool: PgPool,
}

impl Db {
  pub fn new(database_url: &str) -> Result<Db, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Ok(Db {
      pool: Pool::builder().build(manager)?
    })
  }

  pub fn conn(&self) -> Result<PgPooledConnection, &'static str> {
    self.pool.get().map_err(|_| "Error getting database connection")
  }
}
