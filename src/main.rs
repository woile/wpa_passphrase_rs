use clap::Parser;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use std::process;

/// Generate a WPA PSK from an ASCII passphrase for a SSID
#[derive(Parser, Debug)]
struct Args {
    /// Network name
    ssid: String,

    /// Optional password for the ssid
    password: Option<String>,
}

fn get_psk(ssid: &str, password: &str) -> String {
    let mut buf = [0u8; 32];
    pbkdf2::<Hmac<sha1::Sha1>>(password.as_bytes(), ssid.as_bytes(), 4096, &mut buf);
    hex::encode(buf.as_ref())
}

fn main() {
    let args = Args::parse();
    let password = match args.password {
        Some(pass) => pass,
        None => {
            println!("# reading passphrase from stdin");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line = line.trim_end().to_string();
            if line.len() < 8 || line.len() > 63 {
                eprintln!("Passphrase must be 8..63 characters");
                process::exit(1);
            }
            if line.chars().any(|c| char::is_ascii_control(&c)) {
                eprintln!("Invalid passphrase character");
                process::exit(1);
            }
            line
        }
    };

    let ssid = args.ssid;
    let out = get_psk(&ssid, &password);
    println!("network = {{\n\tssid=\"{ssid}\"\n\tpsk={out}\n}}");
}

#[cfg(test)]
mod test {
    use crate::get_psk;
    use rstest::*;

    #[rstest]
    #[case(
        "simple",
        "pass123456",
        "a8be2bfb553258cf65340658a896710d29cf5c3a7472a89e16bd033cdd3fc9ba"
    )]
    // wpa_passphrase IEEE password
    #[case(
        "IEEE",
        "password",
        "f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e"
    )]
    // wpa_passphrase ThisIsASSID ThisIsAPassword
    #[case(
        "ThisIsASSID",
        "ThisIsAPassword",
        "0dc0d6eb90555ed6419756b9a15ec3e3209b63df707dd508d14581f8982721af"
    )]
    // wpa_passphrase ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    #[case(
        "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "becb93866bb8c3832cb777c2f559807c8c59afcb6eae734885001300a981cc62"
    )]
    fn test_psk2(#[case] ssid: &str, #[case] password: &str, #[case] expected: &str) {
        let hash = get_psk(ssid, password);
        assert_eq!(hash, expected)
    }
}
