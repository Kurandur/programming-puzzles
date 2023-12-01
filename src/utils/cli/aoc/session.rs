use dirs::home_dir;
use std::{fs, path::PathBuf};

const SESSION_FILE_NAME: &str = ".aoc_session";

pub struct SessionManager {
    session_token: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        let session_token = match SessionManager::get_session_file_path() {
            Some(session_file_path) => match fs::read_to_string(&session_file_path) {
                Ok(content) => Some(content),
                Err(_) => None,
            },
            None => None,
        };

        SessionManager {
            session_token: session_token,
        }
    }

    fn get_session_file_path() -> Option<PathBuf> {
        let path = home_dir().map_or_else(
            || {
                // Handle the None case here
                eprintln!("Error: Path not available");
                // You can return a default path or perform some other error handling logic
                None
            },
            |home| Some(home.join(SESSION_FILE_NAME).to_path_buf()),
        );
        path
    }

    pub fn get_session_token(&self) -> Result<String, String> {
        self.session_token
            .clone()
            .ok_or_else(|| "No session token available".into())
    }

    pub fn set_session_token(&mut self, token: String) -> Result<(), std::io::Error> {
        if let Some(session_file_path) = SessionManager::get_session_file_path() {
            self.session_token = Some(token.clone());
            return fs::write(session_file_path, token);
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error: Path not available",
        ))
    }
}
