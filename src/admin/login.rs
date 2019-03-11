
#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
pub mod auth;

#[path="../parse.rs"]
pub mod parse;

pub fn controller(json: serde_json::value::Value) -> String {

    //make base db dirs
    files::db_dir();

    //chekc if auth feilds exists in json
    if
        json["username"].is_null() ||
        json["password"].is_null()
    {
        return server::error("invalid_request-params".to_string());
    }

    //extract feilds from json as strings
    let user = parse::clean(json["username"].to_string());
    let pass = parse::clean(json["password"].to_string());

    //check if user exists
    if check_user(user.clone()) == false {
        return server::error("invalid-user".to_string())
    }

    //read user file
    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    let read = files::read_file(path);
    let hash = auth::hash256(pass.to_string());

    //check user password
    if hash != read[0] {
        return server::error("access-denied".to_string())
    }

    let hold = process_user(user);

    return success(hold.user,hold.token)

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

fn process_user(user:String) -> Token {

    let token = auth::token_issue(user.clone());
    let path = files::pathify("\\fuc\\tokens\\ ".to_string() + &user.clone() + &".ftok".to_string());
    if files::check_file(path.clone()) == true {
        files::delete_file(path.clone());
    }
    files::make_file(path.clone());
    let hold = vec![token.user,token.time,token.disguise];
    files::write_file(path.clone(),hold);

    Token {
        success:true,
        user:user.clone(),
        token:token.token
    }

}

//********************************************************
//common

#[derive(Serialize)]
struct Token{
    success:bool,
    user:String,
    token:String
}

fn success(user:String,token:String) -> String {
    stringify(Token {
        success:true,
        user:user,
        token:token
    })
}

fn stringify(hold: Token) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}
