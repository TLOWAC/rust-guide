mod config;
mod controller;

use actix_web::{App, HttpServer};
use dotenvy::from_filename;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        // NOTE: RUST_ENV[local/development/production] 환경변수 설정
        let env_path = format!(".env.{}", env::var("RUST_ENV").unwrap());

        from_filename(env_path).unwrap();

        let config = crate::config::Settings::from_env().unwrap();

        HttpServer::new(|| {
                let app = App::new().service(controller::get_user);
                return app;
        })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}
