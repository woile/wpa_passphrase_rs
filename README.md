# wpa_passphrase_rs

> Utility to create WPA PSK for raspberry pi

## About

Using `wpa_passphrase` on a mac can be tricky, my idea is to have it available on Nix,
to provision the raspberries from any Unix OS.

## Installation

## Usage

With nix:
```sh
nix shell 'github:woile/wpa_passphrase_rs'
```

Then it will be available in your shell

```sh
wpa_passphrase [ ssid ] [ passphrase ]
```

## Resources

- https://linux.die.net/man/8/wpa_passphrase
- https://www.rfc-editor.org/rfc/rfc2898#appendix-A.2
- https://github.com/aosp-mirror/platform_external_wpa_supplicant/blob/master/wpa_passphrase.c

## Comparison

Run this command in your shell to have a linux based wpa_passphrase

```
wpa_passphrase() {
  docker run \
    --rm \
    --interactive \
    --tty \
    "backplane/wpa_passphrase" \
    "$@"
}
```