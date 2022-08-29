# totp cli

Very simple totp cli implementation, the real job is done by [totp-cli](https://github.com/fosskers/totp-lite)

## Usage

```
Usage: totp SECRET <STEP> <DIGITS> <ALG> <TIMESTAMP>

  SECRET    = Unpadded base32 (RFC4648) encoded secret
  STEP      = Seconds of step, defaults to 30
  DIGITS    = Number of digits in the result, defaults to 6
  ALG       = Algorithm sha1, sha256 or sha512, defaults to sha1
  TIMESTAMP = UTC Unix timestamp in seconds, defaults to current

  Note: practically all services use sha1 as default

Example:
  totp KZXW6ZDPN4
  766369

```
