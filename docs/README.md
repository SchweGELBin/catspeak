# catspeak
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/SchweGELBin/catspeak/total)](https://github.com/SchweGELBin/catspeak/releases)
[![GitHub License](https://img.shields.io/github/license/SchweGELBin/catspeak)](../LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/SchweGELBin/catspeak)](https://github.com/SchweGELBin/catspeak/releases/latest)
[![AUR Version](https://img.shields.io/aur/version/catspeak)](https://aur.archlinux.org/packages/catspeak)

- [Cowsay](https://github.com/piuccio/cowsay) like program of a speaking cat
- Available in my [nur expressions](https://github.com/SchweGELBin/nur-expressions) repo

```
$ catspeak "meow~"

 |\ /|
/ . . \
\ _-_ /      meow~
 |   |     _
 |    \   /
 |     |-/
 |_|_ /
```


## Build

### General
``` bash
git clone https://github.com/SchweGELBin/catspeak.git
cd catspeak
cargo build --release
```
Binary will be at `./target/release/catspeak`

### Nix
``` bash
nix build github:SchweGELBin/catspeak
```
Binary will be at `./result/bin/catspeak`


## Run

### General
``` bash
git clone https://github.com/SchweGELBin/catspeak.git
cd catspeak
cargo run -- "meow~"
```

### Nix
``` bash
nix run github:SchweGELBin/catspeak -- "meow~"
```
