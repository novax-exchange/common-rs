use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct PostgresConn {
    port:   i16,
    user:   String,
    pwd:    String,
    host:   String,
    db:     String
}

/// Define postgres connection
/// parameters
/// 
impl PostgresConn {
    /// new create an new 
    /// instance for PostgresConn
    /// Paramter sequence:
    /// * String (user)
    /// * String (pwd)
    /// * String (host)
    /// * String (database)
    /// * i16    (port)
    pub fn new(param: (String, String, String, String, i16)) -> Self {
        let (user, pwd, host, db, port) = param;
        Self {
            port, user, pwd, host, db
        }
    }
    fn to_sql(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
            self.user, self.pwd, self.host, self.port, self.db)
    }
}

pub async fn create_connection(my_conn: PostgresConn, num_cn: u32) -> Result<PgPool, sqlx::Error> {
    let database_url = my_conn.to_sql();
    PgPoolOptions::new()
        .max_connections(num_cn)
        .connect(&database_url).await

}

// re-export
pub use sqlx;

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn should_postgres_connection_ok() -> Result<(), sqlx::Error> {
        let postgres_cn = PostgresConn::new(
            ("myuser".to_string(), "mypassword".to_string(), 
            "localhost".to_string(), "mydatabase".to_string(), 5432)
        );
        let cn = create_connection(postgres_cn, 5).await;
        assert_eq!(cn.is_ok(), true);
        // check simple sql
        let row: (i64,) = sqlx::query_as("SELECT $1").bind(150_i64).fetch_one(&cn?).await?;
        assert_eq!(row.0 == 150, true);
        Ok(())
    }
}
