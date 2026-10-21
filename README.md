# device-mapper-tests

## Crates

- device-mapper-tests: Tests for the framework itself.
- writeboost-tests: Tests for [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost). Make sure `dm-writeboost` is installed.
- wb-command-tests: Tests for `writeboost-cli` from [dm-writeboost-tools](https://github.com/akiradeveloper/dm-writeboost-tools). Make sure `dm-writeboost` and `writeboost-cli` are installed.

## Requirements for the framework

These modules must be installed in the system to get the framework working.

- xfs, xfsprogs
- crypt target, cryptsetup
- CRYPTO_USER
- CRYPTO_USER_API_SKCIPHER

Run `cryptsetup benchmark -c aes-xts-plain64 -s 512` to make sure the cryptsetup is correctly installed.
