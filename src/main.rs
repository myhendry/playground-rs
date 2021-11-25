#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod server;
mod http;
mod demo;

use std::env;
use server::Server;
use demo::Player;

fn main() {
    // let request = Request{method: "ali", path: "http://1", query_string: "xxx"};
    // println!("{:?}", request);
    //let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // -> Result<String, VarError> 
    // let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // let public_path = env::var("PUBLIC_PATH");
    // println!("{:?}", public_path);
    // let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();

    let p1 = Player::new("Darren");
    println!("{}", p1.shout());
}

