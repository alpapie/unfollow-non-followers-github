
use std::{env, process::exit};
use serde_json::Error; 

use reqwest::{
    blocking::Client,
    header::{ACCEPT, USER_AGENT},
};


pub fn make_request(url:&str,token: &str,page: u64) ->  String {
    let http_request = Client::new();
    let http_response =http_request.get(format!("{}{}",url,page))
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36")
        .header(ACCEPT, "application/vnd.github+json")
        .header("Authorization","Bearer ".to_owned()+token )
        .send();
    if http_response.is_ok() {
        let response = http_response.ok().unwrap();
        if response.status().as_u16()!=200 {
            println!("Bad credentials");
            exit(0);
        }
        return response.text().unwrap_or("".to_string());
    }
    println!("Bad credentials");
    exit(0);
}

pub fn get_user(url:String)->Vec<User> {
    let token = get_token();

    let mut i = 1;
    let mut all_users: Vec<User> = Vec::new();

    loop {
        let response = make_request(&url, &token, i);
        let users_list: Result<Vec<User>, Error> = serde_json::from_str(&response);

        match users_list {
            Ok(users) if !users.is_empty() => {
                all_users.extend(users); 
                i += 1; 
            }
            _ => break,
        }
    }
    all_users
}

pub fn get_token() -> String {
    let Ok(token) = env::var("TOKEN") else{
        println!("No token find");
        exit(0);
    };
    token
}

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize,PartialEq, Eq,Hash,Clone)]
pub struct User{
    pub login : String,
    pub id: u64,
    pub avatar_url: String
}