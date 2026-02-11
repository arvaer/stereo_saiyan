use deadpool_diesel::postgres::{Manager, Pool};

use crate::errors::InfraError;

#[derive(Clone)]
pub struct DbPool {
    pool: Pool,
}

impl DbPool {
    pub fn new(db_url: &str) -> Result<Self, InfraError> {
        let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
        let pool = Pool::builder(manager)
            .build()
            .map_err(|e| InfraError::Pool(e.to_string()))?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}
