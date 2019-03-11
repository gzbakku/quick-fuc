
#[path="./parse.rs"]
mod parse;

#[path="./files.rs"]
mod files;

#[path="./list.rs"]
mod list;

mod common;

const LOG:bool = false;

use std::thread;

//main function
#[allow(dead_code)]
pub fn insert(uid:String,val:String){

    common::tell("started => process_words_vector".to_string(),LOG);

    base_dir(uid.clone());

    let a = arrayrify(val);

    for word in a {

        let score = score(word.clone());
        let group = group(score.clone());

        // dictonerify(uid.clone(),word.clone(),score.clone());
        // mapify(uid.clone(),score.clone(),group.clone());

        let hold_uid = uid.clone();
        let hold_word = word.clone();
        let hold_score = score.clone();
        let hold_group = group.clone();
        let mut threads = Vec::new();

        threads.push(thread::spawn(move || {
            dictonerify(hold_uid.clone(),hold_word,hold_score.clone());
            mapify(hold_uid,hold_score,hold_group);
        }));

        for i in threads {
            let _ = i.join();
        }

        common::tell("completed => process_words_vector".to_string(),LOG);

    }

}

//put word in the dict with score as file name
#[allow(dead_code)]
fn dictonerify(uid:String,word:String,score:String){
    let path = files::pathify("\\fuc\\words\\".to_string()) +
               &uid +
               &"\\dict\\".to_string() +
               &score.to_string() +
               &".fpoi".to_string();
    if files::check_file(path.clone()) == false {
        files::make_file(path.clone());
        files::write_file(path.clone(),vec![word]);
    }
}

//put word scores into maps and groups
#[allow(dead_code)]
fn mapify(uid:String,score:String,group:String) {

    //add group id to the map
    let map_path = files::pathify("\\fuc\\words\\".to_string()) +
                   &uid +
                   &"\\map\\map.fump".to_string();


    //println!("map_path : {:?}",map_path);

    files::make_file(map_path.clone());
    let mut read = files::read_file(map_path.clone());
    let pos = read.iter().position(|r| r == &score.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(score.clone());
            read.sort();
            files::write_file(map_path.clone(),read);
        }
    }

    //add score to group
    let group_path = files::pathify("\\fuc\\words\\".to_string()) +
                     &uid +
                     &"\\map\\".to_string() +
                     &group +
                     &".fgup".to_string();

    files::make_file(group_path.clone());
    let mut read = files::read_file(group_path.clone());
    let pos = read.iter().position(|r| r == &score.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(score.clone());
            read.sort();
            files::write_file(group_path.clone(),read);
        }
    }

}

//make base dirs
#[allow(dead_code)]
fn base_dir(uid:String){
    let hold = vec![
        files::pathify("\\fuc\\words\\".to_string()),
        files::pathify("\\fuc\\words\\".to_string() + &uid + &"\\".to_string()),
        files::pathify("\\fuc\\words\\".to_string() + &uid + &"\\dict\\".to_string()),
        files::pathify("\\fuc\\words\\".to_string() + &uid + &"\\map\\".to_string())
    ];
    for i in hold {
        files::make_dir(i.to_string());
    }
}

//split words into a vector of Strings
#[allow(dead_code)]
pub fn arrayrify(s:String) -> Vec<String> {
    let mut h = Vec::new();
    for i in s.split(" ") {
        h.push(i.to_string());
    }
    h
}

//find the score of the word
#[allow(dead_code)]
pub fn score(w:String) -> String {
    let mut s: i64 = 0;
    let mut f:i64 = 0;
    let mut count = 0;
    for i in w.bytes() {
        if count == 0 {
            f += i64::from(i);
        } else {
            s += i64::from(i);
        }
        count += 1;
    }
    f.to_string() + &s.to_string()
}

//find what collection to put the word in
#[allow(dead_code)]
pub fn group(s:String) -> String {

    let n = s.clone().parse::<u32>().unwrap();

    if s.len() == 3 {
        if n < 500 {
            return "500".to_string();
        }
        if n > 500 {
            return "1000".to_string();
        }
    }

    let l = last_3(s.clone());

    fn last_3(s:String) -> u32 {
        let mut pool = Vec::new();
        for i in s.chars() {
            pool.push(i);
        }
        let mut l = pool.len();
        l -= 1;
        let mut dool = Vec::new();
        let mut count = 1;
        while count < 4 {
            dool.push(pool[l]);
            count += 1;
            l -= 1;
        }
        let h = dool[2].to_string() + &dool[1].to_string() + &dool[0].to_string();
        h.parse::<u32>().unwrap()
    }

    let g = groupify(s.clone());

    fn groupify(s:String) -> u32 {
        let mut pool = Vec::new();
        for i in s.chars() {
            pool.push(i);
        }
        let mut l = pool.len();
        l -= 1;
        let mut count = 1;
        while count < 4 {
            pool.remove(l);
            count += 1;
            l -= 1;
        }
        let mut h = String::new();
        for i in pool {
            h.push(i);
        }
        h.parse::<u32>().unwrap()
    }

    let b;
    if l < 500 {
        b = g.to_string() + &"500".to_string();
    } else {
        b = (g + 1).to_string() + &"000".to_string();
    }

    return b

}
