use crate::secrets::Secrets;
use crate::strava::Strava;
use log::{debug, info};
use std::error::Error;
use std::io::stdin;
use std::path::PathBuf;
use url::Url;

pub struct Uploader {
    secrets: Secrets,
}

impl Uploader {
    pub fn new(secrets_path: &str) -> Self {
        Uploader {
            secrets: Secrets::new(secrets_path),
        }
    }

    pub fn is_configured(&self) -> bool {
        self.secrets.refresh_token().is_some()
    }

    pub fn prompt_user_client_id() -> Result<String, Box<dyn Error>> {
        let mut client_id = String::new();
        println!("Please enter your Strava client ID:");
        stdin().read_line(&mut client_id)?;
        client_id.pop();
        Ok(client_id)
    }

    pub fn prompt_user_client_secret() -> Result<String, Box<dyn Error>> {
        let mut client_secret = String::new();
        println!("Please enter your Strava client secret:");
        stdin().read_line(&mut client_secret)?;
        client_secret.pop();
        Ok(client_secret)
    }

    pub fn prompt_code(client_id: &str) -> Result<String, Box<dyn Error>> {
        println!("Please open this URL and validate authorization for your Strava application to interact with your Strava account:");
        println!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://localhost&approval_prompt=force&scope=activity:write", client_id);
        println!("Once authorized, please paste here the URL you have been redirected to :");
        let mut redirected_url = String::new();
        stdin().read_line(&mut &mut redirected_url)?;
        redirected_url.pop();
        let parsed_url = Url::parse(&redirected_url)?;
        Ok(parsed_url
            .query_pairs()
            .find_map(|(k, v)| if k == "code" { Some(v) } else { None })
            .ok_or("Can not extract code from redirected URL")?
            .to_string())
    }

    pub fn configure_secrets(
        &mut self,
        client_id_cli: Option<&str>,
        client_secret_cli: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let client_id = if let Some(x) = client_id_cli {
            String::from(x.trim())
        } else {
            Self::prompt_user_client_id()?
        };

        let code = Self::prompt_code(&client_id)?;

        let client_secret = if let Some(x) = client_secret_cli {
            String::from(x.trim())
        } else {
            Self::prompt_user_client_secret()?
        };

        let (refresh_token, access_token) =
            Strava::get_initial_tokens(&client_id, &client_secret, &code)?;
        // We are now sure that passed credentials are valid, store them
        self.secrets.set_client_id(client_id);
        self.secrets.set_client_secret(client_secret);
        self.secrets.set_refresh_token(refresh_token);
        self.secrets.set_access_token(access_token);
        self.secrets.store()
    }

    pub fn update_secrets(&mut self) -> Result<(), Box<dyn Error>> {
        let (refresh_token, access_token) = Strava::renew_tokens(
            self.secrets.client_id().as_ref().unwrap(),
            self.secrets.client_secret().as_ref().unwrap(),
            self.secrets.refresh_token().as_ref().unwrap(),
        )?;
        self.secrets.set_refresh_token(refresh_token);
        self.secrets.set_access_token(access_token);
        self.secrets.store()
    }

    pub fn push_activity(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        info!("Asking upload for file {:?}", path);
        let access_token = self.secrets.access_token().as_ref().unwrap();
        debug!("Using access token {}", access_token);
        Strava::upload_activity(path, access_token)
    }
}
