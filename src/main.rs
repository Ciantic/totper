use base32::decode;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp_custom, Sha1, Sha256, Sha512};

fn print_usage() {
    println!("Usage: totper SECRET <STEP> <DIGITS> <ALG> <TIMESTAMP>");
    println!("");
    println!("       or pipe the input parameters");
    println!("");
    println!("  SECRET    = Unpadded Base32 (RFC4648) encoded secret");
    println!("  STEP      = Seconds of step, defaults to 30");
    println!("  DIGITS    = Number of digits in the result, defaults to 6");
    println!("  ALG       = Algorithm SHA1, SHA256 or SHA512, defaults to SHA1");
    println!("  TIMESTAMP = UTC Unix timestamp in seconds, defaults to current");
    println!("");
    println!("  Note: practically all services use SHA1 as default");
    println!("");
    println!("Examples: ");
    println!("  echo KZXW6ZDPN4 | totper");
    println!("  766369");
    println!("");
    println!("  totper KZXW6ZDPN4");
    println!("  766369");
    println!("");
    println!("Repository: https://github.com/Ciantic/totp-cli");
}

fn try_do(args: Vec<String>) -> Result<String, String> {
    let secret: Vec<u8> = args
        .get(0)
        .filter(|f| f != &"")
        .map(|f| decode(base32::Alphabet::RFC4648 { padding: false }, &f))
        .flatten()
        .ok_or("Secret was not given in right format")?;

    let step = args
        .get(1)
        .map_or(Ok(30), |f| f.parse::<u64>())
        .map_err(|_| "Step must be integer")?;

    let digits = args
        .get(2)
        .map_or(Ok(6), |f| f.parse())
        .map_err(|_| "Digits must be given as integer")?;

    let alg: String = args.get(3).map_or("sha1".to_string(), |f| f.to_string());

    let seconds = args.get(4).map_or_else(
        || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        },
        |f| f.parse().unwrap(),
    );

    match alg.to_lowercase().as_str() {
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
        _ => Err("Algorithm must be SHA1, SHA256 or SHA512".to_string()),
    }
}

fn main() {
    let read_from_stdin = !atty::is(atty::Stream::Stdin);

    // If user is trying to supply cli arguments and stdin arguments at the same
    // time. It is ambigous which method to use
    if read_from_stdin && std::env::args().len() > 1 {
        eprintln!("Error: Piping and giving command line arguments at the same time.");
        println!("");
        print_usage();
        std::process::exit(1);
    }

    // Read arguments from cli or stdin
    let args: Vec<String> = if read_from_stdin {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Foo");
        line.trim().split(" ").map(ToOwned::to_owned).collect()
    } else {
        std::env::args().skip(1).collect()
    };

    if args.len() == 0 {
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
