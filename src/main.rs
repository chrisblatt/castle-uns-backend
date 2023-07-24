/*
Author: Chris Blatt
Purpose: Backend to add DNS records to a db
*/

#![feature(decl_macro)]

use rocket::get;
use rocket::routes;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Navigate to http://localhost:8000/check/<type your Coin> to check  details about this crypto  !"
}

#[get("/healthz")]
fn health() -> &'static str {
    "healthy"
}

#[get("/check/<coin>")]
fn check(coin: &RawStr) -> Result<String, Box<dyn std::error::Error>> {
    let request_url =format!("https://api.coingecko.com/api/v3/coins/{}", coin);
    println!("{}", request_url);
    let resp = reqwest::blocking::get(request_url)?;
    if resp.status().is_success(){
        Ok(format!("{} ", resp.text().unwrap()))
    }
    else{
        let response = resp.text().unwrap();
        Ok(format!("{} is not a coin!", response))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, check, health]).launch();
}