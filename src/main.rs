mod github_api;
use dotenv::dotenv;
use server_github::ServerGithub;
mod server_github;
mod helper;

fn main() {
    dotenv().ok(); 
    let port="5000".to_owned();
   ServerGithub::new(port).start();
}
