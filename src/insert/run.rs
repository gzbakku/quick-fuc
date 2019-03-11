#[path="../server.rs"]
mod server;

#[path="../files.rs"]
mod files;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
mod parse;

#[path="./index/mod.rs"]
mod index;

#[path="../list.rs"]
mod list;

use serde_json::value::Value;
use std::thread;

pub fn controller(json: Value) -> String {

    //*********************************************************************************************
    //check json keys
    if
        json["token"] == "null" ||
        json["username"] == "null" ||
        json["address"] == "null" ||
        json["docs"] == "null"
    {
        return server::error("invalid_request-params".to_string())
    }

    let token = parse::clean(json["token"].to_string());
    let username = parse::clean(json["username"].to_string());
    let address = parse::clean(json["address"].to_string());

    if auth::token_verify(username,token) == false {
        return server::error("access_denied".to_string())
    }

    files::db_dir();

    //*********************************************************************************************
    //get coll ids
    let cuid = parse::cuid(address.clone());
    let ccid = parse::ccid(address.clone());

    //*********************************************************************************************
    //this section converts the stirng of docs into a vec of docs
    let docs_array_result = serde_json::to_value(&json["docs"]);
    let mut docs_array:Value = serde_json::from_str("[]").unwrap();
    let mut docs_array_successfull = true;
    match docs_array_result {
        Ok(n)=>{
            docs_array = n;
        }
        Err(error)=>{
            println!("error : {:?}",error);
            docs_array_successfull = false;
        }
    }
    if docs_array_successfull == false {
        return server::error("invalid_docs-invalid_json_schema".to_string());
    }
    let docs_unchecked = documentify(docs_array);
    let docs = check_docs(cuid.clone(),docs_unchecked);

    //*********************************************************************************************
    //indexify

    let hold_cuid = cuid.clone();
    let hold_docs = docs.clone();

    //index::process_docs(cuid.clone(),ccid,docs.clone());

    let thread_index = thread::spawn(move || {
        index::process_docs(hold_cuid,ccid,hold_docs);
    });

    //*********************************************************************************************
    //insert docs

    //insert(cuid,docs.clone());

    let hold_cuid = cuid.clone();
    let hold_docs = docs.clone();

    let thread_insert = thread::spawn(move || {
        insert(hold_cuid,hold_docs);
    });

    let _ = thread_index.join();
    let _ = thread_insert.join();

    //println!("index : {:#?}",index);

    server::success()

}

fn insert(cuid:String,docs:Vec<Value>){

    let docs_dir = files::pathify("\\fuc\\docs\\".to_string());
    let coll_docs_list = parse::md5(cuid.clone() + &"-docs".to_string());

    for i in docs {

        let doc = i.to_string();
        let doc_id = parse::md5(doc.clone());
        let doc_path = docs_dir.clone() +
                       &cuid.clone() +
                       &"_".to_string() +
                       &doc_id.clone() +
                       &".json".to_string();

        files::make_file(doc_path.clone());
        files::write_file(doc_path,vec![doc.clone()]);

        let hold_coll_docs_list_id = coll_docs_list.clone();
        let hold_cuid = cuid.clone();

        //list::insert(hold_coll_docs_list_id,doc_id);

        thread::spawn(move || {
            let list_file = list::insert(hold_coll_docs_list_id,doc_id.clone());
            if list_file.len() > 0 {
                insert_ref(hold_cuid,doc_id,list_file);
            }
        });

    }

}

//this function checks if the docs already exists
fn check_docs(cuid:String,docs:Vec<Value>) -> Vec<Value> {

    let docs_dir = files::pathify("\\fuc\\docs".to_string());
    let mut coll : Vec<Value> = Vec::new();

    for i in docs {

        let doc = i.to_string();
        let doc_id = parse::md5(doc.clone());
        let doc_path = docs_dir.clone() +
                       &cuid.clone() +
                       &"_".to_string() +
                       &doc_id.clone() +
                       &".json".to_string();

        if files::check_file(doc_path) == false {
            coll.push(i.clone());
        }

    }

    return coll;
}

//this function takes a json array of docs and convert it into a vec of docs
fn documentify(a:Value) -> Vec<Value> {
    let array_hold = a.as_array();
    let array_len;
    match array_hold {
        Some(n)=>{
            array_len = n.len();
        },
        None=>{
            array_len = 0;
        }
    }
    let mut count = 0;
    let mut pool = Vec::new();
    while count < array_len {
        pool.push(a[count].to_owned());
        count += 1;
    }
    pool
}

fn insert_ref(cuid:String,doc_id:String,val:String){

    let docs_dir = files::pathify("\\fuc\\docs\\".to_string());
    let file_name = cuid + &"_".to_string() + &doc_id + &".ref".to_string();
    let file_path = docs_dir + &file_name;

    if files::check_file(file_path.clone()) == false {
        files::make_file(file_path.clone());
    }

    let mut read = files::read_file(file_path.clone());
    let pos = read.iter().position(|r| r == &val);
    match pos {
        Some(_)=>{},
        None=>{
            read.push(val);
        }
    }

    files::write_file(file_path,read);

}
