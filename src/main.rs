use actix_web::{get, web, App, HttpServer, Responder, Result};
use rand::Rng;
use vin;
pub mod structs;
use chrono::Datelike;


#[get("/calculateDisselUsageForDistance")]
async fn dissel_calculation(req_params: web::Query<structs::CalculationDisselDetails>) -> Result<impl Responder> {
    let date_now = chrono::Local::now();
    let current_year = date_now.year();
    if req_params.yearOfProduction < 1885 || req_params.yearOfProduction > current_year as i16 {
        return Err(actix_web::error::ErrorBadRequest(format!("Year of production must be between 1886 and {current_year}", current_year = current_year)));
    }
    let distance = req_params.distance;
    let fuel_usage_per_100km = req_params.fuelUsagePer100KM;
    let fuel_usage = (distance / 100) as i16 * fuel_usage_per_100km;
    let response = structs::DieselCalculationResponse {
        fuelUsage: fuel_usage,
    };
    Ok(web::Json(response))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn injector_fail_probability(req_params: web::Query<structs::ProbabilityUnitInjectorFail>) -> Result<impl Responder> {
    if vin::check_validity(&req_params.VIN).is_err() {
        return Err(actix_web::error::ErrorBadRequest("Invalid VIN"));
    }
    let injector_fail_probability = rand::thread_rng().gen_range(0.0..1.0);
    let injector_fail_probability_rounded = f32::trunc(injector_fail_probability  * 100.0) / 100.0;
    let response = structs::ProbabilityUnitInjectorFailResponse {
        failProbability: injector_fail_probability_rounded,
    };
    Ok(web::Json(response))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(dissel_calculation)
            .service(injector_fail_probability)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}