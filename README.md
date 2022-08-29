# totper - the very simple totp cli

Very simple totp cli implementation, the real job is done by [totp-lite](https://github.com/fosskers/totp-lite)

## Usage

```
Usage: totper SECRET <STEP> <DIGITS> <ALG> <TIMESTAMP>

       or pipe the input parameters

  SECRET    = Unpadded Base32 (RFC4648) encoded secret
  STEP      = Seconds of step, defaults to 30
  DIGITS    = Number of digits in the result, defaults to 6
  ALG       = Algorithm SHA1, SHA256 or SHA512, defaults to SHA1
  TIMESTAMP = UTC Unix timestamp in seconds, defaults to current

  Note: practically all services use SHA1 as default

Examples:
  echo KZXW6ZDPN4 | totper
  766369

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
