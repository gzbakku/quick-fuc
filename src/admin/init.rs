
#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
pub mod auth;

#[path="../parse.rs"]
pub mod parse;

pub fn controller(json: serde_json::value::Value) -> String {

    //read user file
    let path = files::pathify("\\fuc\\keys\\register.fkey".to_string());
    if files::check_file(path.clone()) == true {
        return server::error("db-already_initiated".to_string());
    }

    //make base db dirs
    files::db_dir();

    let base = parse::clean(json["base"].to_string());
    let hash = auth::hash256(base);
    let key = auth::hash256(hash.clone());

    make_key(path.clone(),key);
    return success(hash);

}

//********************************************************
//modular functions

fn make_key(p:String,key:String) {
    files::make_file(p.clone());
    files::write_file(p.clone(),vec![key]);
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Key {
    pub success:bool,
    pub key:String
}

#[allow(dead_code)]
pub fn success(key:String) -> String {
    stringify(Key {
        success:true,
        key:key
    })
}

#[allow(dead_code)]
fn stringify (hold: Key) -> String {
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
