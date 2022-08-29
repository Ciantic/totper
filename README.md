# totper - the very simple totp cli

Very simple totp cli implementation, the real job is done by [totp-lite](https://github.com/fosskers/totp-lite)

## Usage

```
Usage: totper SECRET <STEP> <DIGITS> <ALG> <TIMESTAMP>

  SECRET    = Unpadded base32 (RFC4648) encoded secret
  STEP      = Seconds of step, defaults to 30
  DIGITS    = Number of digits in the result, defaults to 6
  ALG       = Algorithm sha1, sha256 or sha512, defaults to sha1
  TIMESTAMP = UTC Unix timestamp in seconds, defaults to current

  Note: practically all services use sha1 as default

Example:
  totper KZXW6ZDPN4
  766369

```

## Install

```
cargo install totper
```

## How to store secrets

I use this with GPG based [pass](https://www.passwordstore.org/) password manager, which keeps the secrets in GPG encrypted files.

## TODO:

-   Read from stdin
-   Maybe change the order of arguments, because secret should be read from STDIN practically always?
