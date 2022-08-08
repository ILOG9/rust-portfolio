use dotenv::dotenv;
#[macro_use]
extern crate dotenv_codegen;
mod modules;
mod server;

fn main() {
    println!("---------------------------------------------");
    println!("|CHLABS ~ Tecnología Chilena");
    println!(
        "software: {} V{}",
        dotenv!("APP_NAME"),
        dotenv!("APP_VERSION")
    );
    println!("---------------------------------------------");
    dotenv().ok();

    match server::actix_web() {
        Ok(()) => (),
        Err(e) => println!("Server error {}", e),
    }
}
