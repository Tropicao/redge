use clap::{App, Arg};
use log::info;
use redge::device::Device;
use redge::uploader::Uploader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let device = Device::new()?;
    for entry in device.get_activities() {
        info!("Found activity {}", entry.to_str().unwrap());
    }

    env_logger::init();
    let args = App::new("redge")
        .author("Alexis Lothoré")
        .about("Utility to upload activities from sport device to Strava")
        .arg(
            Arg::with_name("client_id")
                .short("i")
                .long("client_id")
                .takes_value(true)
                .help("Client ID to use if new secrets configuration is needed"),
        )
        .arg(
            Arg::with_name("client_secret")
                .short("s")
                .long("client_secret")
                .takes_value(true)
                .help("Client secret to use if new secrets configuration is needed"),
        )
        .get_matches();

    let mut uploader = Uploader::new(".secrets");
    if !uploader.is_configured() {
        uploader.configure_secrets(args.value_of("client_id"), args.value_of("client_secret"))?;
    } else {
        info!("Credentials are configured");
        uploader.update_secrets()?
    }

    let activity = device.get_last_activity();
    uploader.push_activity(activity)
}
