use sqlx::mysql::MySqlPool;

pub struct MysqlConn {
    port:   i16,
    user:   String,
    pwd:    String,
    host:   String,
    db:     String
}

/// Define mysql connection
/// parameters
/// 
impl MysqlConn {
    /// new create an new 
    /// instance for MysqlConn
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
        format!("mysql://{}:{}@{}:{}/{}",
            self.user, self.pwd, self.host, self.port, self.db)
    }
}

pub async fn create_connection(my_conn: MysqlConn) -> Result<MySqlPool, sqlx::Error> {
    let database_url = my_conn.to_sql();
    MySqlPool::connect(&database_url).await
}

// re-export
pub use sqlx;

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use log;
    use sqlx::Row;

    #[tokio::test]
    async fn should_mysql_connection_ok() -> Result<(), sqlx::Error> {
        env_logger::init();
        let mysql_cn = MysqlConn::new(
            ("myuser".to_string(), "mypassword".to_string(), 
            "localhost".to_string(), "mydatabase".to_string(), 3306)
        );
        // ensure application run
        log::info!("parsing database uri ");
        std::env::set_var("DATABASE_URL", &mysql_cn.to_sql());
        // ensure application run
        let cn = create_connection(mysql_cn).await;
        assert_eq!(cn.is_ok(), true);
        // check simple sql
        let row = sqlx::query(r#"select 1 as id"#).fetch_one(&cn?).await?;
        assert_eq!(row.get::<'_, i32, _>("id") == 1, true);
        Ok(())

    } 

}
