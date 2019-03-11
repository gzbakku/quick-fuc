

mod files;
mod parse;

//insert the doc md5
#[allow(dead_code)]
pub fn insert(list_name:String,val:String) -> String {
    let fult = get_fult(list_name);
    let mut read = files::read_file(fult.clone());
    let pos = read.iter().position(|r| r == &val);
    match pos {
        Some(_n)=>{
            return String::new();
        },
        None => {
            read.push(val);
            files::write_file(fult.clone(),read);
            return fult;
        }
    }
}

//get fult here
#[allow(dead_code)]
pub fn get_fult(list_name:String) -> String {

    let list_dir = files::pathify("\\fuc\\lists\\".to_string());

    //get fump
    make_map(list_name.clone());
    let fump_path = list_dir.clone() + &list_name.clone() + &".fump".to_string();
    let read_fump = files::read_file(fump_path.clone());

    //println!("fump path : {:?}",fump_path);

    //read fult
    let fult_name = parse::md5(list_name.clone() + &"_".to_string() + &read_fump[0].to_string());
    let fult_path = list_dir.clone() + &fult_name + &".fult".to_string();
    let read_fult = files::read_file(fult_path.clone());

    //println!("fult path : {:?}",fump_path);

    if read_fult.len() > 99 {

        let next_fult_name = parse::md5(
            list_name.clone() +
            &"_".to_string() +
            &(fult_name.parse::<i32>().unwrap() + i32::from(100)).to_string()
        );
        let next_fult_path = list_dir.clone() + &next_fult_name + &".fult".to_string();

        //println!("next_fult_path : {:?}",next_fult_path);

        files::write_file(fump_path.clone(),vec![next_fult_name]);
        files::make_file(next_fult_path.clone());

        return next_fult_path;

    } else {

        files::make_file(fult_path.clone());
        return fult_path;

    }

}

//make map before asking for fult
#[allow(dead_code)]
pub fn make_map(list_name:String){

    let fump = files::pathify("\\fuc\\lists\\".to_string()) + &list_name.clone() + &".fump".to_string();
    let fult = files::pathify("\\fuc\\lists\\".to_string()) +
    &parse::md5(list_name + &"_100".to_string()) +
    &".fult".to_string();

    //println!("map_fump : {:?}",fump);
    //println!("map_fult : {:?}",fult);

    if files::check_file(fump.clone()) == false {
        files::make_file(fump.clone());
        files::write_file(fump,vec!["100".to_string()]);
    }

    if files::check_file(fult.clone()) == false {
        files::make_file(fult);
    }

}
