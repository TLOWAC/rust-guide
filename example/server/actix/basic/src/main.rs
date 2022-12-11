mod config;
mod controller;

use crate::config::Settings;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::from_filename;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        // NOTE: RUST_ENV[local/development/production] 환경변수 설정
        let env_path = format!(".env.{}", env::var("RUST_ENV").unwrap());
        from_filename(env_path).unwrap();

        let config = Settings::from_env().unwrap();
        let Settings { server, .. } = config;

        env_logger::init_from_env(Env::default().default_filter_or("info"));

        HttpServer::new(|| {
                let app = App::new()
                        .wrap(Logger::new("%a %r %s %T"))
                        .service(controller::get_user);

                return app;
        })
        .bind((server.host, server.port))?
        .run()
        .await
}
