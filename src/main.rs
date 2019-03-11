
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate crypto;
extern crate rand;

extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

//i have no idea what to do with this
// maybe it was used in iron to impliment max body size for incoming data
//use persistent::Read;

use iron::status;
use iron::prelude::*;
use router::Router;
use std::env;
use std::net::TcpListener;

mod admin;
mod query;
mod insert;

//******************************************************
//main

fn main() {

    let args: Vec<String> = env::args().collect();

    let port;

    if args.len() >= 2 {
        let port_object = &args[1];
        if port_object.parse::<u16>().is_ok() {
            port = port_object.parse::<u16>().unwrap();
        } else {
            port = "3000".to_string().parse::<u16>().unwrap();
        }
    } else {
        port = "3000".to_string().parse::<u16>().unwrap();
    }

    if check_port(port.clone()) == true {
        serve(port.to_string());
    } else {
        println!("!!! port in use");
    }

}

fn check_port(port:u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => {return true;},
        Err(_) => {return false;},
    }
}

fn serve(port:String){

    println!("listing on port : {}",port.clone());

    let url = "127.0.0.1:".to_string() + &port.to_string();

    let mut router = Router::new();

    router.get("/", index_controller, "index");
    router.post("/", index_controller, "index");
    router.post("/init", admin::init_router, "init");

    router.post("/user/register", admin::register_router, "register");
    router.post("/user/reset", admin::reset_router, "reset");
    router.post("/user/delete", admin::delete_router, "delete");
    router.post("/user/login", admin::login_router, "login");

    router.post("/query/register", query::register_router, "query_register");
    //router.post("/query/run", query::run_router, "query_run");

    router.post("/insert", insert::insert_router, "insert");

    Iron::new(router).http(url).unwrap();

}

fn index_controller(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "what are you doing here".to_string())))
}
