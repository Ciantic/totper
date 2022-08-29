use base32::decode;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp_custom, Sha1, Sha256, Sha512};

fn print_usage() {
    println!("Usage: totper SECRET <STEP> <DIGITS> <ALG> <TIMESTAMP>");
    println!("");
    println!("  SECRET    = Unpadded base32 (RFC4648) encoded secret");
    println!("  STEP      = Seconds of step, defaults to 30");
    println!("  DIGITS    = Number of digits in the result, defaults to 6");
    println!("  ALG       = Algorithm sha1, sha256 or sha512, defaults to sha1");
    println!("  TIMESTAMP = UTC Unix timestamp in seconds, defaults to current");
    println!("");
    println!("  Note: practically all services use sha1 as default");
    println!("");
    println!("Example: ");
    println!("  totper KZXW6ZDPN4");
    println!("  766369");
    println!("");
    println!("Repository: https://github.com/Ciantic/totp-cli");
}

fn try_do(args: Vec<String>) -> Result<String, String> {
    let secret = args
        .get(1)
        .map(|f| decode(base32::Alphabet::RFC4648 { padding: false }, &f))
        .flatten()
        .ok_or("Secret was not given in right format")?;

    let step = args
        .get(2)
        .map_or(Ok(30), |f| f.parse::<u64>())
        .map_err(|_| "Step must be integer")?;

    let digits = args
        .get(3)
        .map_or(Ok(6), |f| f.parse())
        .map_err(|_| "Digits must be given as integer")?;

    let alg: String = args.get(4).map_or("sha1".to_string(), |f| f.to_string());

    let seconds = args.get(5).map_or_else(
        || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        },
        |f| f.parse().unwrap(),
    );

    match alg.as_str() {
        "sha1" => Ok(totp_custom::<Sha1>(
            step,
            digits,
            secret.as_slice(),
            seconds,
        )),
        "sha256" => Ok(totp_custom::<Sha256>(
            step,
            digits,
            secret.as_slice(),
            seconds,
        )),
        "sha512" => Ok(totp_custom::<Sha512>(
            step,
            digits,
            secret.as_slice(),
            seconds,
        )),
        _ => Err("Algorithm must be sha1, sha256 or sha512".to_string()),
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        print_usage();
        return;
    }
    match try_do(args) {
        Ok(v) => {
            println!("{}", v);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("");
            print_usage();
            std::process::exit(1);
        }
    }
}
