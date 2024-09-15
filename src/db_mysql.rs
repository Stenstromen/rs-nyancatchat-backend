// src/db_mysql.rs

use mysql::*;
use mysql::prelude::*;
use std::env;

pub fn create_pool() -> Pool {
    let host_name = env::var("MYSQL_HOSTNAME").expect("MYSQL_HOSTNAME must be set");
    let data_base = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be set");
    let user_name = env::var("MYSQL_USERNAME").expect("MYSQL_USERNAME must be set");
    let pass_word = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host_name))
        .db_name(Some(data_base))
        .user(Some(user_name))
        .pass(Some(pass_word));

    let pool = Pool::new(opts).unwrap();

    // Initialize the database table if it doesn't exist
    let mut conn = get_connection(&pool).unwrap();
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS msgtable (
            id INT AUTO_INCREMENT PRIMARY KEY,
            user VARCHAR(255),
            messagecontent VARCHAR(255),
            messageiv VARCHAR(255),
            room VARCHAR(255)
        )",
    )
    .unwrap();

    pool
}

pub fn get_connection(pool: &Pool) -> Result<PooledConn> {
    pool.get_conn()
}