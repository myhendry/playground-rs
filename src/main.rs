#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod http;
mod server;
mod website_handler;

mod demo;
mod server2;
mod website_handler2;

use std::{env, collections::HashMap};
use server::Server;
use website_handler::WebsiteHandler;

use demo::{Player, Teacher, Request2, Method2};
use server2::{Server2};
use website_handler2::WebsiteHandler2;

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
    // let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // println!("public path: {}", public_path);

    // let server = Server::new("127.0.0.1:8080".to_string());
    // server.run(WebsiteHandler::new(public_path));

    // let p1 = Player::new("Alvin", Some("apple".to_string()));
    // println!("{}", p1.shout());

    // let t1 = Teacher::new("Jerry", 30);
    // println!("{}", t1.introduce());
    // println!("{}", t1.hey());

    //let r2 = Request2{path: "http://localhost:4000", query_string: Some("hello".to_string()), method: Method2::GET};
    //println!("r2 {:?}", r2)

    // ! Server 2
    // let s1 = Server2::new("127.0.0.1:3000".to_string());
    // s1.run();

    // ! https://stackoverflow.com/questions/62231909/borrowing-mutable-struct-with-fields
    // let mut p2 = Player::new("Jeremy", Some("Chris".to_string()), 32, "John".to_string());
    // p2.profile.amend(20, "Ali");
    // p2.profile.amend(200, "Xavier");
    // p2.profile.amend(2000, "Thor");

    // println!("{:?}", p2);

    // ! HaspMaps
  //let mut hm1 = HashMap::new();
    // hm1.insert("1", "google");
    // hm1.insert("2","meta");
    // hm1.insert("3", "microsoft");
    // println!("{:?}", hm1);

    // // Unwrapping Option<&&str> other than using unwrap
    // let v1 = hm1.get("2");
    // println!("{:?}", v1.unwrap());

    // println!("{}", hm1.len());

    /*
    ! Main   
    1. Listen
    2. HTTP Parse
    3. Handle
    */
    // env! is a macro that reads environment variables at compile time
    // let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // // env::var reads environment variables at run time
    // // todo where does the PUBLIC_PATH appear?
    // let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // // println!("{}", public_path);
    // // todo why not use &str in this case rather than String??
    // let server = Server2::new("127.0.0.1:8080".to_string());
    // server.run(WebsiteHandler2::new(public_path));    
    
    // let s1 = "apple";
    // let s2 = &s1[1..=4];
    // println!("{}", s2);    
    // dbg!(s2);

    // let a1 = [10, 20, 30];
    // let a2 = &a1[..2];
    // println!("{:?}", a2);

    // let mut v1 = vec![1, 2, 3];
    // v1.push(4);
    // println!("{:?}", v1);

    let a1 = [1, 2, 3, 4, 5];
    accept_arr1(&a1); // OK
    accept_arr2(&a1); // OK
    accept_arr2(&a1[1..3]); // OK
    //&[u8] //byte slice

}

fn accept_arr1(a: &[i32; 5]) {
  println!("{:?}", a);
}

fn accept_arr2(a: &[i32]) {
  println!("{:?}", a);
}