#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
mod parse;

pub fn controller(json: serde_json::value::Value) -> String {

    files::db_dir();

    if
        json["key"].is_null() ||
        json["username"].is_null()
    {
        return server::error("invalid_request-params".to_string());
    }

    let key = read_key();
    let hashed_key = auth::hash256(parse::clean(json["key"].to_string()));

    if hashed_key != key {
        return server::error("access-denied".to_string())
    }

    if check_user(parse::clean(json["username"].to_string())) == false {
        return server::error("invalid-user".to_string())
    }

    delete_user(parse::clean(json["username"].to_string()));

    return server::success()

}

//********************************************************
//modular functions

fn check_user(user:String) -> bool {
    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    if files::check_file(path.clone()) == false {
        false
    } else {
        true
    }
}

fn delete_user(user:String) {
    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    files::delete_file(path);
}

pub fn read_key() -> String {
    let path = files::pathify("\\fuc\\keys\\register.fkey".to_string());
    let file = files::read_file(path);
    let hold = &file[0];
    hold.to_string()
}
