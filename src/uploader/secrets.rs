use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::BufReader,
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    client_id: Option<String>,
    client_secret: Option<String>,
    refresh_token: Option<String>,
    access_token: Option<String>,
}

pub struct Secrets {
    secrets_path: String,
    credentials: Credentials,
}

impl Credentials {
    fn new() -> Self {
        Credentials {
            client_id: None,
            client_secret: None,
            refresh_token: None,
            access_token: None,
        }
    }
}

impl Secrets {
    pub fn new(secrets_path: &str) -> Self {
        Secrets {
            secrets_path: String::from(secrets_path),
            credentials: if let Ok(x) = File::open(Path::new(secrets_path)) {
                if let Ok(y) = serde_json::from_reader(BufReader::new(x)) {
                    y
                } else {
                    Credentials::new()
                }
            } else {
                Credentials::new()
            },
        }
    }

    pub fn client_id(&self) -> &Option<String> {
        &self.credentials.client_id
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.credentials.client_id = Some(client_id)
    }

    pub fn client_secret(&self) -> &Option<String> {
        &self.credentials.client_secret
    }

    pub fn set_client_secret(&mut self, client_secret: String) {
        self.credentials.client_secret = Some(client_secret)
    }

    pub fn refresh_token(&self) -> &Option<String> {
        &self.credentials.refresh_token
    }

    pub fn set_refresh_token(&mut self, refresh_token: String) {
        self.credentials.refresh_token = Some(refresh_token)
    }

    pub fn access_token(&self) -> &Option<String> {
        &self.credentials.access_token
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.credentials.access_token = Some(access_token)
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .mode(0o600)
            .write(true)
            .read(true)
            .create(true)
            .open(Path::new(&self.secrets_path))?;
        Ok(serde_json::to_writer(file, &self.credentials)?)
    }
}
