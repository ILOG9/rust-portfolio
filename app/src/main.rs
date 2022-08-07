use colored::*;
use dotenv::dotenv;
#[macro_use]
extern crate dotenv_codegen;
mod modules;
mod server;

fn main() {
    println!("---------------------------------------------");
    println!("{}CHLABS ~ Tecnología Chilena", "|".truecolor(233, 107, 86));
    println!(
        "software: {} {}{}",
        dotenv!("APP_NAME").truecolor(233, 107, 86),
        "V".truecolor(233, 107, 86),
        dotenv!("APP_VERSION").truecolor(233, 107, 86)
    );
    println!("---------------------------------------------");
    dotenv().ok();

    match server::actix_web() {
        Ok(()) => (),
        Err(e) => println!("Error {}", e),
    }
}
