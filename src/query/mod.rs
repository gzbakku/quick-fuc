use iron::status;
use iron::prelude::*;

#[path="../server.rs"]
mod server;

//*********************************************
//local mods

//mod run;
mod register;

pub fn register_router(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = register::controller(json_body);
        },
        Ok(None) => {
            y = server::error("invalid-request".to_string());
        },
        Err(_err) => {
            y = server::error("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

// pub fn run_router(req: &mut Request) -> IronResult<Response> {
//     let json_body = req.get::<bodyparser::Json>();
//     let y;
//     match json_body {
//         Ok(Some(json_body)) => {
//             y = init::controller(json_body);
//         },
//         Ok(None) => {
//             y = server::error("invalid-request".to_string());
//         },
//         Err(_err) => {
//             y = server::error("unknown-error".to_string());
//         }
//     }
//     Ok(Response::with((status::Ok, y)))
// }
