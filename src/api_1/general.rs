extern crate libcalculatorfinance;

use actix_web::{web, HttpResponse};

pub fn convert_from_orig(params: web::Path<(f64, f64)>) -> HttpResponse
{
  HttpResponse::Ok().json(libcalculatorfinance::convert_from_orig(params.0, params.1))
}

