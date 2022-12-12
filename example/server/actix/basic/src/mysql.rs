use diesel::{
        prelude::*,
        r2d2::{self, ConnectionManager},
};

use crate::config::Settings;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> DbPool {
        // NOTE : set up database connection pool

        let config = Settings::from_env().unwrap();

        // 1) database config 가져오기
        let Settings { database, .. } = config;

        // 2) database url 조회하기
        let database_url = database.get_connection_url();

        // 3) mysql connection manager 설정
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);

        // 4) mysql connection pool 생성
        let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
        return pool;
}
