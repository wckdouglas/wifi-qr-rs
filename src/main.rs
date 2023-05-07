/// inspired by https://github.com/lakhanmankani/wifi_qrcode_generator/tree/main
pub use clap::Parser;
use clap::ValueEnum;
use image::Luma;
use log::info;
use qrcode::QrCode;
use std::string::String;

const DEFAULT_IMAGE_PATH: &str = "wifi_qr.png";

/// Generating a QR code for connecting to a WIFI network
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the Stats.json file
    #[clap(short, long)]
    ssid: String,

    /// whether or not the network is hidden
    #[clap(short, long, default_value_t = false)]
    is_hidden: bool,

    /// authentication type
    #[clap(short, long)]
    auth_type: AuthType,

    /// password for the wifi
    #[clap(short, long)]
    password: Option<String>,

    #[clap(short, long, default_value_t = String::from(DEFAULT_IMAGE_PATH))]
    output_image: String,
}

/// this is a hack to print out the enum name
/// see here: https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
#[derive(Debug, Clone, ValueEnum)]
enum AuthType {
    WPA,
    WEP,
    NOPASS,
}

/// Generate a WiFi code for the given parameters.
/// The generated WiFi code can be rendered into a QR code to be scanned to join the network.
/// Args:
///     ssid: Network SSID.
///     is_hidden: Specify if the network is hidden.
///     authentication_type: Specify the authentication type. Supported types: `WPA`, `WEP`, `nopass`.
///     password: Network password. If `authetication_type` is `None`, this argument should be set to `None`.
/// Returns:
///     The WiFi code for the given parameters.
fn wifi_code(
    ssid: String,
    is_hidden: bool,
    authentication_type: AuthType,
    password: Option<String>,
) -> Result<String, String> {
    let password_result: Result<String, String> = match authentication_type {
        AuthType::WEP | AuthType::WPA => match password {
            Some(p) => Ok(p),
            _ => Err("For WPA and WEP, password should be provided".to_string()),
        },
        AuthType::NOPASS => match password {
            Some(_p) => Err("For nopass, no password should be provided".to_string()),
            _ => Ok("nopass".to_string()),
        },
    };
    let password_string = password_result?;
    // wifi qr code format from:
    // https://pocketables.com/2022/01/how-to-format-that-wifi-qr-code-in-plain-text.html
    Ok(format!(
        "WIFI:S:{};T:{:?};P:{};H:{};;",
        ssid, authentication_type, password_string, is_hidden
    ))
}

fn qr(args: Cli) -> Result<(), String> {
    info!(
        "Creating QR code for SSID: \"{}\" with authentication: [{:?}]",
        &args.ssid, &args.auth_type
    );
    let wifi_string = wifi_code(args.ssid, args.is_hidden, args.auth_type, args.password)?;
    // copied from module https://docs.rs/qrcode/latest/qrcode/
    // Encode some data into bits.
    let code = QrCode::new(wifi_string).unwrap();
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // Save the image.
    image.save(&args.output_image).unwrap();
    info!("Writting QR code image to: {}", args.output_image);
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();

    let result = qr(args);
    match result {
        Ok(_) => (),
        Err(err_string) => println!("{}", err_string),
    };
}
