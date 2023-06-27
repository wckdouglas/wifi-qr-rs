use clap::ValueEnum;

/// This is the type of authentication we support
/// this is a hack to print out the enum name
/// see here: https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
#[derive(Debug, Clone, ValueEnum)]
pub enum AuthType {
    /// Wi-Fi Protected Access
    WPA,

    /// Wired Equivalent Privacy
    WEP,

    /// no password is set
    NOPASS,
}
