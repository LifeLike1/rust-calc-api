use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CalculationDisselDetails {
    pub distance: i64,
    pub yearOfProduction: i16,
    pub fuelUsagePer100KM: i16,
}

#[derive(Serialize)]
pub struct DieselCalculationResponse {
    pub fuelUsage: i16
}

#[derive(Deserialize)]
pub struct ProbabilityUnitInjectorFail {
    pub VIN: String,
}

#[derive(Serialize)]
pub struct ProbabilityUnitInjectorFailResponse {
    pub failProbability: f32
}

