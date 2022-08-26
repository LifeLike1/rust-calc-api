# Car calculations REST API in Rust

## Description

The task was aimed at creating simple REST API service with two GET endpoints.
Main required language for this project is Rust.
Built using [Actix](https://actix.rs/) web framework.
All structs are included in `structs.rs` file.
Endpoints and server run is in `main.rs` file.

## Install

### Tools

- Rustup (Recommended - includes needed Cargo package manager)
  https://www.rust-lang.org/tools/install
- Cargo (if installed different tool than Rustup)
  https://doc.rust-lang.org/cargo/getting-started/installation.html

### Run the app

1.  Extract project files to your desired location
2.  Run build command (it will install all dependencies):
    `cargo build`
3.  Run start command:
    ` cargo run`
4.  App is running on `localhost:8080`. You can now check endpoints!

## Endpoints

### Dissel usage for distance

#### Request

Calculates how much fuel is used for a provided distance.
`GET /calculateDisselUsageForDistance`

#### Query params (all required!)

```
distance: i64
yearOfProduction: i16 (Must be in <1886, current_year> range)
fuelUsagePer100KM: i16
```

#### Response

```
Format: JSON
Status: 200
Response body:
{
	fuelUsage:  i16
}
```

### Probability of unit injector fail

#### Request

Calculates probability of failure on the Unit Injector element.
`GET /probabilityOfUnitInjectorFail`

#### Query params (VIN required)

```
VIN: String (Must be correct VIN)
```

#### Response

```
Format: JSON
Status: 200
Response body:
{
	failProbability:  f32 <0, 1> range
}
```

## Dependencies used

- Actix-web
- Serde
- Derive_more
- Rand
- Vin
- Chrono

## References

https://actix.rs/
https://docs.rs/actix-web/latest/actix_web/
https://serde.rs/
https://jeltef.github.io/derive_more/derive_more/
https://docs.rs/vin/latest/vin/
https://docs.rs/chrono/latest/chrono/
