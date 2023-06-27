use crate::models::AuthType;
use clap::Parser;

const DEFAULT_IMAGE_PATH: &str = "wifi_qr.png";

/// Generating a QR code for connecting to a WIFI network
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// wifi network name
    #[clap(short, long)]
    pub ssid: String,

    /// whether or not the network is hidden
    #[clap(short, long, default_value_t = false)]
    pub is_hidden: bool,

    /// authentication type
    #[clap(short, long, value_enum, default_value_t=AuthType::WPA)]
    pub auth_type: AuthType,

    /// password for the wifi
    #[clap(short, long)]
    pub password: Option<String>,

    #[clap(short, long, default_value_t = String::from(DEFAULT_IMAGE_PATH))]
    pub output_image: String,
}
