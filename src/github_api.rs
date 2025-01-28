use std::collections::HashSet;

use crate::helper::{get_user, User};

pub struct GithubApi{
    pub follower_url: String,
    pub following_url: String,

}

impl GithubApi{
    pub fn new()->Self{
        GithubApi{
            follower_url:"https://api.github.com/user/followers?per_page=100&page=".to_string(),
            following_url:"https://api.github.com/user/following?per_page=100&page=".to_string(),
        }
    }

    pub fn fetch_follwer(&self)->Vec<User>{
        get_user(self.follower_url.clone())
    }

    pub fn fetch_follwing(&self)->Vec<User>{
        get_user(self.following_url.clone())
    }

    pub fn you_not_following_u(&self)->Vec<User>{
        let followers=self.fetch_follwer();
        let following= self.fetch_follwing();

        let followers_set: HashSet<_> = followers.iter().collect();
        let following_set: HashSet<_> = following.iter().collect();
    
        let not_following_back: Vec<_> = following_set
            .difference(&followers_set)
            .cloned()
            .cloned()
            .collect();
        not_following_back
    }
}

