

#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
pub mod auth;

#[path="../parse.rs"]
pub mod parse;

pub fn controller(json: serde_json::value::Value) -> String {

    server::success()

}
