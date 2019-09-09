extern crate actix_web;
use actix_web::{web, App, HttpServer};

pub mod api_1;

fn main()
{
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/status").route(web::get().to(api_1::status::status)))
            .service(
                web::resource("/convert_from_orig/{value}/{from_value}")
                    .route(web::get().to(api_1::general::convert_from_orig)),
            )
    })
    .bind("127.0.0.1:8888")
    .unwrap()
    .run()
    .unwrap();
}
