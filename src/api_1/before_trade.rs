extern crate libcalculatorfinance;

use actix_web::{web, HttpResponse};

pub fn calculate_shares_recommended(params: web::Path<(f64, f64, f64, f64)>) -> HttpResponse
{
    HttpResponse::Ok().json(libcalculatorfinance::calculate_shares_recommended(params.0, params.1, params.2, params.3))
}

pub fn calculate_leveraged_contracts(param: web::Path<i32>) -> HttpResponse
{
    HttpResponse::Ok().json(libcalculatorfinance::calculate_leveraged_contracts(*param))
}
