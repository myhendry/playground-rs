#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod server;
mod http;
mod website_handler;
mod demo;

use std::env;
use server::Server;
use demo::{Player, Teacher};
use website_handler::WebsiteHandler;

fn main() {
    // let request = Request{method: "ali", path: "http://1", query_string: "xxx"};
    // println!("{:?}", request);
    //let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // -> Result<String, VarError> 
    // let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // let public_path = env::var("PUBLIC_PATH");
    // println!("{:?}", public_path);

    // ! L58 Working with Environmental Variables
    // cargo expand | code -   // todo cargo expand not working
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

    // let p1 = Player::new("Darren", Some("apple".to_string()));
    // println!("{}", p1.shout());

    // let t1 = Teacher::new("Jerry", 30);
    // println!("{}", t1.introduce());
    // println!("{}", t1.hey());
}

