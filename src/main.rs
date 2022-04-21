use std::{path, io};

mod models;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

static path_to_data: &'static str = "../data/dataset_B_05_2020.csv";

#[get("/<name>")]
fn predict(name: String) -> String {
     
     
     return "".to_string();
 }


 
 #[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/predict", routes![predict])


}
