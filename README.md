# wifi-qr-rs #

This is a simple rust program that generates a QR code for connecting to a wifi network when scanned.

The program is heavily inspired by [wifi_qrcode_generator](https://github.com/lakhanmankani/wifi_qrcode_generator).

## Installation ##

```
$ git clone https://github.com/wckdouglas/wifi-qr-rs.git
$ cd wifi-qr-rs
$ cargo install --path .
$ wifi-qr-rs -h
Generating a QR code for connecting to a WIFI network

Usage: wifi-qr-rs [OPTIONS] --ssid <SSID> --auth-type <AUTH_TYPE>

Options:
  -s, --ssid <SSID>                  Path to the Stats.json file
  -i, --is-hidden                    whether or not the network is hidden
  -a, --auth-type <AUTH_TYPE>        authentication type [possible values: wpa, wep, nopass]
  -p, --password <PASSWORD>          password for the wifi
  -o, --output-image <OUTPUT_IMAGE>  [default: wifi_qr.png]
  -h, --help                         Print help
  -V, --version                      Print version
```


## Usage ## 

```
$ wifi-qr-rs \
    --ssid my_network_name \
    --auth-type wpa \
    --password my_password \
    --output-image my_qr_code.png
[2023-05-13T17:24:02Z INFO  wifi_qr_rs] Creating QR code for SSID: "my_network_name" with authentication: [WPA]
[2023-05-13T17:24:02Z INFO  wifi_qr_rs] Writting QR code image to: my_qr_code.png
```