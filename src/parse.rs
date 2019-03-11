
use crypto::md5::Md5;
use crypto::digest::Digest;


#[allow(dead_code)]
pub fn md5(s:String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&s);
    hasher.result_str()
}

#[allow(dead_code)]
pub fn clean(s:String) -> String {
    let mut hold = String::new();
    for i in s.chars() {
        for j in i.to_string().bytes(){
            if j != 34 {
                hold.push_str(&i.to_string());
            }
        }
    }
    hold
}

#[allow(dead_code)]
//a is for address
//this function cllects all the collection names form the address and retuns a md5 hash or a false as a string
pub fn ccid(a:String) -> String {

    //println!("address : {:?}",a);
    let pods: Vec<&str> = a.split("|||").collect();
    if pods.len() == 0 {
        return "false".to_string();
    }

    //println!("pods : {:?}",pods);
    let mut initials: Vec<String> = Vec::new();
    for i in pods {
        let h: Vec<&str> = i.split("===>").collect();
        initials.push(h[0].to_string());
    }
    if initials.len() == 0 {
        return "false".to_string();
    }

    //println!("initials : {:?}",initials);
    let mut pull = String::new();
    for i in initials{
        pull.push_str(&i)
    }

    md5(pull)

}


//a is for address
//this function checks if the collection address is valid
#[allow(dead_code)]
pub fn cuid(a:String) -> String {

    //println!("address : {:?}",a);
    let pods: Vec<&str> = a.split("|||").collect();
    if pods.len() == 0 {
        return "false".to_string();
    }

    //println!("pods : {:?}",pods);
    let mut indi:Vec<Vec<&str>> = Vec::new();
    for i in pods {
        indi.push(i.split("===>").collect());
    }

    //println!("indi : {:?}",indi);
    let last_pod = &indi[indi.len() - 1];
    if last_pod.len() > 1 {
        return "false".to_string();
    }

    md5(a)

}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Index {
    pub valid:bool,
    pub index_type:String,
    pub index_id:String,
    pub equal:Vec<String>,
    pub search:Vec<String>,
    pub order:Vec<String>,
}

//this function verifies a query string and returns a struct
#[allow(dead_code)]
pub fn index(q:String) -> Index {

    //println!("index : {:?}",q);

    let mut bb = Index{
        valid:true,
        index_type:String::from("equal"),
        index_id:md5(q.clone()),
        equal:Vec::new(),
        search:Vec::new(),
        order:Vec::new()
    };

    let pods:Vec<&str> = q.split(".").collect();

    //println!("pods : {:?}",pods);

    let mut tags:Vec<Vec<&str>> = Vec::new();
    for i in pods {
        tags.push(i.split("_").collect());
    }
    //println!("tags : {:?}",tags);

    for i in tags{
        if i.len() == 2 {
            if i[1] == "equal" {
                bb.equal.push(i[0].to_string());
            } else
            if i[1] == "search" {
                bb.search.push(i[0].to_string());
                bb.index_type = "search".to_string();
            } else
            if i[1] == "order" {
                bb.order.push(i[0].to_string());
                bb.index_type = "order".to_string();
            } else {
                bb.valid = false;
            }
        } else {
            bb.valid = false;
        }
    }

    //println!("bb : {:#?}",bb);

    bb

}






//
