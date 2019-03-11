//***************************************************
//common server responses

//---------------------------------------
//success

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Success {
    pub success:bool
}

#[allow(dead_code)]
pub fn success() -> String {
    stringify_success(Success {
        success:true
    })
}

#[allow(dead_code)]
fn stringify_success(hold: Success) -> String {
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

//---------------------------------------
//error

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Error {
    pub success:bool,
    pub error:String
}

#[allow(dead_code)]
pub fn error(err:String) -> String {
    stringify_error(Error {
        success:false,
        error:String::from(err)
    })
}

#[allow(dead_code)]
fn stringify_error(hold: Error) -> String {
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
