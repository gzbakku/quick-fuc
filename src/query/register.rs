

#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
pub mod auth;

#[path="../parse.rs"]
pub mod parse;

#[derive(Debug)]
#[allow(dead_code)]
struct Index {
    pub valid:bool,
    pub index_type:String,
    pub equal:Vec<String>,
    pub search:Vec<String>,
    pub order:Vec<String>,
}

pub fn controller(json: serde_json::value::Value) -> String {

    //check json keys
    if
        json["token"] == "null" ||
        json["username"] == "null" ||
        json["query"] == "null" ||
        json["address"] == "null"
    {
        return server::error("invalid_request-params".to_string())
    }

    let token = parse::clean(json["token"].to_string());
    let username = parse::clean(json["username"].to_string());
    let address = parse::clean(json["address"].to_string());
    let query = parse::clean(json["query"].to_string());

    if auth::token_verify(username,token) == false {
        return server::error("access_denied".to_string())
    }

    if parse::index(query.to_string()).valid == false {
        return server::error("invalid_query".to_string());
    } 

    insert(parse::ccid(address.clone()),query.to_string());
    server::success()

}

fn insert(ccid:String,query:String){

    let index_dir = files::pathify(String::from("\\fuc\\index\\"));
    let file_path = index_dir + &ccid + &".fui".to_string();

    if files::check_file(file_path.clone()) == false{
        files::make_file(file_path.clone());
        files::write_file(file_path,vec![query]);
        return
    }

    let mut read = files::read_file(file_path.clone());
    let pos = read.iter().position(|r| r == &query);
    match pos{
        Some(_)=>{},
        None=>{
            read.push(query);
            files::write_file(file_path,read);
        }
    }

}
