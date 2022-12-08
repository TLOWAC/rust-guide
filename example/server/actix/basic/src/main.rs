mod config;
mod controller;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        dotenv().ok();

        let config = crate::config::Settings::from_env().unwrap();

        HttpServer::new(|| {
                let app = App::new().service(controller::get_user);
                return app;
        })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}
