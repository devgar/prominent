#![feature(proc_macro_hygiene, decl_macro)]

use std::env;
#[macro_use] extern crate rocket;
use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

fn parse_port(v: String) ->  Result<u16, env::VarError> {
  Ok(v.parse::<u16>().expect("PORT must be a valid u16 number"))
}

fn main() {
  let port = env::var("PORT").and_then(parse_port).unwrap_or(8000);

  let config = Config::build(Environment::Staging)
    .port(port).finalize().expect("Not a valid config");

  rocket::custom(config).mount("/", routes![index]).launch();
}
