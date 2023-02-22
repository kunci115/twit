#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};

mod constants;
mod like;
mod response;
mod bacot;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();


    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(bacot::list)
        .service(bacot::get)
        .service(bacot::create)
        .service(bacot::delete)
        .service(like::list)
        .service(like::plus_one)
        .service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}


