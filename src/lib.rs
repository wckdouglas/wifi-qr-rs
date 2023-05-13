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
///
/// # Example
///
/// ```
/// use wifi_qr_rs::wifi_code;
/// use wifi_qr_rs::models::AuthType;
///
/// let ssid = String::from("my_wifi_network");
/// let wifi_url = wifi_code(
///     ssid,
///     false,
///     AuthType::WPA,
///     Some(String::from("my_password")),
/// );
/// let expected_url = "WIFI:S:my_wifi_network;T:WPA;P:my_password;H:false;;".to_string();
/// assert_eq!(wifi_url.unwrap(), expected_url);
/// ```
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

/// Making a QR code from a string
/// copied from module https://docs.rs/qrcode/latest/qrcode/
///
/// # Arguments
/// `string` - the string to be encoded
/// `output_image` - path to the output QR code image
pub fn make_qr_code_image_from_string(string: String, output_image: String) -> Result<(), String> {
    // Encode some data into bits.
    let code = QrCode::new(string).map_err(|e| e.to_string())?;
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // Save the image.
    image.save(&output_image).map_err(|e| e.to_string())?;
    info!("Writting QR code image to: {}", output_image);
    Ok(())
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
    make_qr_code_image_from_string(wifi_string, output_image)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(String::from("my_wifi_network"),false,AuthType::WPA,Some(String::from("my_password")),"WIFI:S:my_wifi_network;T:WPA;P:my_password;H:false;;".to_string())]
    #[case(String::from("my_wifi_network"),false,AuthType::WEP,Some(String::from("my_password")),"WIFI:S:my_wifi_network;T:WEP;P:my_password;H:false;;".to_string())]
    #[case(String::from("my_wifi_network"),true,AuthType::NOPASS,None,"WIFI:S:my_wifi_network;T:NOPASS;P:nopass;H:true;;".to_string())]
    fn test_wifi_code(
        #[case] ssid: String,
        #[case] is_hidden: bool,
        #[case] auth_type: AuthType,
        #[case] password: Option<String>,
        #[case] expected_url: String,
    ) {
        let wifi_url = wifi_code(ssid, is_hidden, auth_type, password);
        assert_eq!(wifi_url.unwrap(), expected_url);
    }

    #[test]
    #[should_panic]
    fn test_wifi_code_nopassword_but_password_provided() {
        let _fail_result = wifi_code(
            String::from("my_wifi_network"),
            false,
            AuthType::NOPASS,
            Some(String::from("my_password")),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_wifi_code_need_password_but_none_given() {
        let _fail_result =
            wifi_code(String::from("my_wifi_network"), false, AuthType::WPA, None).unwrap();
    }
}
