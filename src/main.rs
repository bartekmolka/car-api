#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 

// importing packages
extern crate rocket;

// adding usings
use rocket::request::{Form};
use rand;
use rand::Rng;

// create a new dto for the first route
#[derive(FromForm)]
struct Fuel {
    distance: u8,
    yearofproduction: u16,
    fuelusageper100km: f32,
}

// create a new dto for the second route
#[derive(FromForm)]
struct VIN {
    vin: String,
}

//adding first route
#[get("/calculateDisselUsageForDistance?<fuel..>")]
fn fuel_consumption(fuel: Form<Fuel>) -> String {
    let fuel = fuel;
    let distance = fuel.distance;
    let _year = fuel.yearofproduction;
    let usage = fuel.fuelusageper100km;
    let consumption = (distance as f32 * usage)/100.0;
    return format!("Your fuel consumption will be equal = {}", consumption);
    
}

//adding second route
#[get("/probabilityOfUnitInjectorFail?<vin..>")]
fn probability_of_failure(vin: Form<VIN>) -> String{
    let mut rng = rand::thread_rng();
    return format!("Probability of unit injector fail is equal to {}", rng.gen_range(0, 100) as f32/100.0);
}

//server setup
fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![fuel_consumption, probability_of_failure])
}

//starting the server
fn main() {
    rocket().launch();
}

// test links`
// http://localhost:8000/calculateDisselUsageForDistance?distance=108&yearofproduction=1999&fuelusageper100km=7.6
// http://localhost:8000/probabilityOfUnitInjectorFail?vin=WDBRF40J43F433102