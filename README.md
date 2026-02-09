# rm-pokemon
[![rm1](https://img.shields.io/badge/rM1-supported-green)](https://remarkable.com/store/remarkable)
[![rm2](https://img.shields.io/badge/rM2-supported-green)](https://remarkable.com/store/remarkable-2)
[![rmpp](https://img.shields.io/badge/rMPP-supported-green)](https://remarkable.com/store/overview/remarkable-paper-pro)
[![rmppm](https://img.shields.io/badge/rMPPM-supported-green)](https://remarkable.com/products/remarkable-paper/pro-move)
[![vellum](https://img.shields.io/badge/vellum-rm--pokemon-purple)](https://vellum.delivery/#/package/rm-pokemon/)

Display the Pokemon sprite that matches the codename from your reMarkable's firmware.

## Usage

```bash
./rm-pokemon              # Display the Pokemon sprite
./rm-pokemon --name-only  # Output only the Pokemon name
./rm-pokemon --version    # Show version
```

To use with fastfetch: `rm-pokemon | fastfetch --file-raw -`

## Install

### Installation via [Vellum package manager](https://github.com/vellum-dev/vellum)

`vellum install rm-pokemon`

### Manual

Download the binary for your device from [releases](https://github.com/rmitchellscott/rm-pokemon/releases):
- `rm-pokemon-armv7` - reMarkable 1/2
- `rm-pokemon-aarch64` - reMarkable Paper Pro (Move)

## Build

```bash
cross build --release --target armv7-unknown-linux-gnueabihf
cross build --release --target aarch64-unknown-linux-gnu
```

## Credits

Pokemon sprites from [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts) (MIT License).
