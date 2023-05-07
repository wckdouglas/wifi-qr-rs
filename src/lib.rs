pub mod cli;
pub mod models;

use image::Luma;
use log::info;
use qrcode::QrCode;

use crate::models::AuthType;

/// Generate a WiFi code for the given parameters.
///
/// # Arguments
/// * `ssid`: Network SSID.
/// * `is_hidden`: Specify if the network is hidden.
/// * `authentication_type`: Specify the authentication type. Supported types: [AuthType]
/// * `password`: Network password. If `authetication_type` is `None`, this argument should be set to `None`.
///
/// # Returns
/// * The WiFi code for the given parameters
pub fn wifi_code(
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

/// Generate a WiFi code for the given parameters and make it into a QR code image
/// The resultant QR code can be scanned to join the network.
///
/// # Arguments:
/// * `ssid`: Network SSID.
/// * `is_hidden`: Specify if the network is hidden.
/// * `authentication_type`: Specify the authentication type. Supported types: [AuthType]
/// * `password`: Network password. If `authetication_type` is `None`, this argument should be set to `None`.
/// * `output_image`: the qr code image output file path
pub fn qr(
    ssid: String,
    is_hidden: bool,
    authentication_type: AuthType,
    password: Option<String>,
    output_image: String,
) -> Result<(), String> {
    info!(
        "Creating QR code for SSID: \"{}\" with authentication: [{:?}]",
        &ssid, &authentication_type
    );
    let wifi_string = wifi_code(ssid, is_hidden, authentication_type, password)?;
    // copied from module https://docs.rs/qrcode/latest/qrcode/
    // Encode some data into bits.
    let code = QrCode::new(wifi_string).map_err(|e| e.to_string())?;
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // Save the image.
    image.save(&output_image).map_err(|e| e.to_string())?;
    info!("Writting QR code image to: {}", output_image);
    Ok(())
}
