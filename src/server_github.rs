use std::{io::Write, net::TcpListener, process::exit};

use crate::{github_api::GithubApi, helper::{get_token, User}};

pub struct ServerGithub {
    pub port: String,
    pub data: Vec<User>,
}

impl ServerGithub {
    pub fn new(port: String) -> Self {
        Self { port,data:Vec::new() }
    }

    pub fn start(&mut self) {
        let Ok(listener) = TcpListener::bind("127.0.0.1:".to_owned() + &self.port) else {
            println!("error in:");
            exit(0)
        };
        println!("\r\r\rhttp://localhost:{}", self.port);
        for stream in listener.incoming() {
            println!("Fetching data...");
            self.data=GithubApi::new().you_not_following_u();
            println!("Fetching Done");
            let mut steam = stream.unwrap();
            steam.write_all(self.parse().as_bytes()).unwrap();
            steam.flush().unwrap();
        }
    }

    pub fn parse(&self) -> String {
        let token = get_token();
        let head = r#"
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/sweetalert2@11/dist/sweetalert2.min.css">
            <script src="https://cdn.jsdelivr.net/npm/sweetalert2@11"></script>
            <div class="user-list">
                <h2>Unfollow non-followers</h2>
                <ul>
        "#.to_string();

        let mut body = String::new();
        for user in &self.data {
            body += &format!(
                r#"<li class="user-card">
                <a href="https://github.com/{}" target="_blank">
                    <img src="{}" alt="User Avatar" class="user-avatar">
                    <span class="user-name">{}</span></a>
                    <button class="unfollow-btn" data-user-name="{}">Unfollow</button>
                </li>"#,user.login,
                user.avatar_url, user.login, user.login
            );
        }

        let script = format!(
            r#"
            </ul>
        </div>
        <script>
            const token = '{}';
            
            document.querySelectorAll('.unfollow-btn').forEach((button) => {{
                button.addEventListener('click', (event) => {{
                    const userName = event.target.dataset.userName;
                    
                    Swal.fire({{
                        title: 'Unfollow User',
                        text: 'Are you sure you want to unfollow this user?',
                        icon: 'warning',
                        showCancelButton: true,
                        confirmButtonColor: '#3085d6',
                        cancelButtonColor: '#d33',
                        confirmButtonText: 'Yes, unfollow',
                        cancelButtonText: 'Cancel',
                    }}).then((result) => {{
                        if (result.isConfirmed) {{
                            const userCard = button.closest('.user-card');
                            userCard.classList.add('removed');
                            setTimeout(() => userCard.remove(), 300);
                            unfollowUser(userName);
                        }}
                    }});
                }});
            }});
    
            async function unfollowUser(userName) {{
                try {{
                    const response = await fetch(`https://api.github.com/user/following/${{userName}}`, {{
                        method: 'DELETE',
                        headers: {{
                            'Authorization': `Bearer ${{token}}`,
                            'Content-Type': 'application/json',
                        }},
                    }});
    
                    if (response.ok) {{
                        console.log(response)
                        Swal.fire({{
                            icon: 'success',
                            title: 'Unfollowed',
                            text: `User ${{userName}} has been unfollowed.`,
                        }});
                    }} else {{
                        const errorData = await response.json();
                        console.log(errorData)
                        Swal.fire({{
                            icon: 'error',
                            title: 'Unfollow Failed',
                            text: errorData.message || 'An error occurred.',
                        }});
                    }}
                }} catch (error) {{
                 console.log(error)
                    Swal.fire({{
                        icon: 'error',
                        title: 'Network Error',
                        text: 'Could not connect to the server. Please try again later.',
                    }});
                }}
            }}
        </script>
        "#,
            token
        );

        let styles = r#"
        <style>
            body {
                font-family: 'Arial', sans-serif;
                margin: 0;
                padding: 0;
                background-color: #f4f4f9;
                color: #333;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
            }
    
            ul {
                max-height: 80vh;
                overflow-y: scroll;
            }
            a{
                text-decoration: none;
    align-items: center;
    display: flex
;}
            .user-card.removed {
                opacity: 0;
                transform: translateX(-50%);
                transition: opacity 0.3s ease, transform 0.3s ease;
            }
    
            .user-list {
                width: 100%;
                max-width: 500px;
                background: #fff;
                padding: 20px;
                border-radius: 12px;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                text-align: center;
            }
    
            .user-list h2 {
                margin-bottom: 20px;
                font-size: 1.5rem;
                color: #222;
            }
    
            .user-card {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 10px 15px;
                border: 1px solid #ddd;
                border-radius: 8px;
                margin-bottom: 10px;
                background-color: #fafafa;
                transition: transform 0.2s ease, box-shadow 0.2s ease;
            }
    
            .user-card:hover {
                transform: translateY(-4px);
                box-shadow: 0 6px 10px rgba(0, 0, 0, 0.1);
            }
    
            .user-avatar {
                width: 50px;
                height: 50px;
                border-radius: 50%;
                object-fit: cover;
            }
    
            .user-name {
                flex-grow: 1;
                margin-left: 15px;
                font-size: 1.1rem;
                color: #555;
                text-align: left;
            }
    
            .unfollow-btn {
                background: #ff5252;
                color: #fff;
                border: none;
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 0.9rem;
                cursor: pointer;
                transition: background 0.2s ease;
            }
    
            .unfollow-btn:hover {
                background: #ff3333;
            }
        </style>
        "#;

        let content = format!("{}{}{}{}", head, body, script, styles);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        response
    }
}
