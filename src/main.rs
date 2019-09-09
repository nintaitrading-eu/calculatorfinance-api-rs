extern crate actix_web;
use actix_web::{web, App, HttpServer};

pub mod api_1;

fn main()
{
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/status").route(web::get().to(api_1::status::status)))

            /* General */
            .service(
                web::resource("/calculate_percentage_of/{value}/{from_value}")
                    .route(web::get().to(api_1::general::calculate_percentage_of)))
            .service(
                web::resource("/convert_from_orig/{price}/{exchange_rate}")
                    .route(web::get().to(api_1::general::convert_from_orig)))
            .service(
                web::resource("/convert_to_orig/{converted_price}/{exchange_rate}")
                    .route(web::get().to(api_1::general::convert_to_orig)))

            /* Before trade */
            // TBD

            /* After trade */
            // TBD
    })
    .bind("127.0.0.1:8888")
    .unwrap()
    .run()
    .unwrap();
}
