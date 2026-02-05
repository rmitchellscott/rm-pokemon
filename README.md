# rm-pokemon

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
