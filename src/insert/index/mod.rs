
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

#[path="../../words.rs"]
mod words;

#[path="../../group.rs"]
mod group;

#[path="../../common.rs"]
mod common;

use serde_json::value::Value;
use std::thread;

const LOG:bool = false;

pub fn process_docs(cuid:String,ccid:String,docs:Vec<Value>){

    common::tell("started => process_docs_vector".to_string(),LOG);

    let index_vec_string = files::read_file(
        files::pathify("\\fuc\\index\\".to_string() + &ccid + &".fui".to_string())
    );

    if index_vec_string.len() == 0{
        return
    }

    let index = parse_index(index_vec_string);
    let mut thread_pool = Vec::new();

    for i in docs {
        let cuid_hold = cuid.clone();
        let index_hold = index.clone();
        thread_pool.push(thread::spawn(move || {
            process_doc(cuid_hold,index_hold,i);
        }));
    }

    for i in thread_pool {
        let _ = i.join();
    }

    common::tell("completed => process_docs_vector".to_string(),LOG);

}

fn process_doc(cuid:String,index:Vec<parse::Index>,doc:Value){
    common::tell("started => process_doc_thread".to_string(),LOG);
    let mut threads = Vec::new();
    for i in index{
        let cuid_hold = cuid.clone();
        let doc_hold = doc.clone();
        threads.push(thread::spawn(move || {
            process_index(cuid_hold,i,doc_hold);
        }));
    }
    for i in threads {
        let _ = i.join();
    }
    common::tell("completed => process_doc_thread".to_string(),LOG);
}

fn process_index(cuid:String,query:parse::Index,doc:Value){

    common::tell("started => process_index_thread".to_string(),LOG);

    let mut path = "collection_id===>".to_string() + &cuid;
    path.push_str(&"&&&index_id===>".to_string());
    path.push_str(&query.clone().index_id);
    path.push_str(&"+++".to_string());

    let mut refs:Vec<String> = Vec::new();

    for i in query.equal{
        let tag = i.to_string();
        let mut h = "(".to_string();
        h.push_str(&tag.clone());
        h.push_str(&",".to_string());
        h.push_str(&parse::clean(doc[tag].to_string()));
        h.push_str(&")".to_string());
        path.push_str(&h);
    }

    let doc_id = parse::md5(doc.to_string());

    if query.index_type == "equal" {
        let list_id = parse::md5(path.clone());
        refs.push(list::insert(list_id.clone(),doc_id.clone()));
    }

    if query.index_type == "search" {
        let tag = &query.search[0].to_string();
        let val = parse::clean(doc[tag.clone()].to_string());
        let get_search_ref = searchify(
            query.index_id.clone(),
            cuid.clone(),
            path.clone(),
            tag.to_string(),
            val,
            doc_id.clone()
        );
        for i in get_search_ref{
            refs.push(i.to_string());
        }
    }

    if query.index_type == "order" {

        let tag = &query.order[0].to_string();
        let extract = doc[tag].as_f64();
        let val;
        match extract {
            Some(n)=>{
                val = n;
            },
            None=>{
                val = 0.0
            }
        }

        refs.push(orderify(
            path.clone(),
            tag.to_string(),
            val,
            doc_id.clone()
        ));

    }

    insert_ref(cuid,doc_id,refs);

    common::tell("completed => process_index_thread".to_string(),LOG);

    //println!("refs : {:?}",refs);

}

fn orderify(base:String,tag:String,val:f64,doc_id:String) -> String {

    let mut hold = base;

    let mut h = "(".to_string();
    h.push_str(&tag.clone().clone());
    h.push_str(&",".to_string());
    h.push_str(&val.clone().to_string());
    h.push_str(&")".to_string());
    hold.push_str(&h);

    let uid = parse::md5(hold.clone());

    group::insert(uid,val.to_string(),doc_id.clone())

}

fn searchify(index_id:String,cuid:String,base:String,tag:String,val:String,doc_id:String) -> Vec<String> {

    let words_vec:Vec<String> = words::arrayrify(val.clone());
    let uid = cuid.clone() + &index_id;
    let mut refs:Vec<String> = Vec::new();

    words::insert(uid.clone(),val.clone());

    let get_indi_refs = indi(base.clone(),tag.clone(),doc_id.clone(),words_vec.clone());

    for i in get_indi_refs {
        refs.push(i.to_string());
    }

    fn indi(base:String,tag:String,doc_id:String,words_vec:Vec<String>) -> Vec<String> {

        let mut refs:Vec<String> = Vec::new();

        for i in words_vec {
            let mut query = base.clone();
            let mut h = "(".to_string();
            h.push_str(&tag.clone());
            h.push_str(&",".to_string());
            h.push_str(&i.to_string());
            h.push_str(&")".to_string());
            query.push_str(&h);
            let list_id = parse::md5(query);
            refs.push(list::insert(list_id,doc_id.clone()));
        }

        refs

    }

    let get_chain_refs = chain(base.clone(),tag.clone(),doc_id.clone(),words_vec.clone());

    for i in get_chain_refs {
        refs.push(i.to_string());
    }

    fn chain(base:String,tag:String,doc_id:String,words_vec:Vec<String>) -> Vec<String> {

        let mut hold = base;
        let mut refs:Vec<String> = Vec::new();

        for i in words_vec {
            let mut h = "(".to_string();
            h.push_str(&tag.clone());
            h.push_str(&",".to_string());
            h.push_str(&i.to_string());
            h.push_str(&")".to_string());
            hold.push_str(&h);
            let list_id = parse::md5(hold.clone());
            refs.push(list::insert(list_id,doc_id.clone()));
        }

        refs

    }

    refs

}

//this function converts a vec of stirng of index to a vec of parsed index sturct
fn parse_index(a:Vec<String>) -> Vec<parse::Index> {
    let mut pool: Vec<parse::Index> = Vec::new();
    for i in a{
        pool.push(parse::index(i));
    }
    pool
}

fn insert_ref(cuid:String,doc_id:String,val:Vec<String>){

    let docs_dir = files::pathify("\\fuc\\docs\\".to_string());
    let file_name = cuid + &"_".to_string() + &doc_id + &".ref".to_string();
    let file_path = docs_dir + &file_name;

    if files::check_file(file_path.clone()) == false {
        files::make_file(file_path.clone());
    }

    let mut read = files::read_file(file_path.clone());

    for i in val {
        let pos = read.iter().position(|r| r == &i.to_string());
        match pos {
            Some(_)=>{},
            None=>{
                read.push(i.to_string());
            }
        }
    }

    files::write_file(file_path,read);

}
