/// inspired by https://github.com/lakhanmankani/wifi_qrcode_generator/tree/main
use clap::Parser;
use wifi_qr_rs::cli::Cli;
use wifi_qr_rs::qr;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();

    let result = qr(
        args.ssid,
        args.is_hidden,
        args.auth_type,
        args.password,
        args.output_image,
    );
    match result {
        Ok(_) => (),
        Err(err_string) => println!("{}", err_string),
    };
}
