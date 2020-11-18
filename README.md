# colonize

A Dwarf-Fortress/Rimworld-like game written in Rust.

***See the [changelog] for what's new in the most recent release.***

![colonize-screenshot](https://i.imgur.com/YI68SsY.jpg "Colonize - Game scene")

# Table of Contents

* [Compiling & Running From Source](#compiling--running-from-source)
* [Contributing](#contributing)
* [License](#license)

## Compiling & Running From Source
### Prerequisites

* [rust](https://www.rust-lang.org)

### Enable Fast Compiles (Optional)

For fast iterative compiles, follow the [instructions in the Bevy book](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)
on how to setup fast compiles.

### Compiling

Build & run:
```sh
cargo run --release
```

### Compiling for WASM

Setup:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

Build & run:
```
cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/release/colonize.wasm
```
Serve project dir to browser. i.e.
```
python3 -m http.server
```

## Contributing

Contributions are always welcome!
If you have an idea for something to add (code, documentation, tests, examples,
etc.) fell free to give it a shot.

Please read [CONTRIBUTING.md][contributing] before you start contributing.

## License

Colonize is distributed under the terms of the GPLv3 license or later.

```
Copyright (C) 2016-2020 Nikita Pekin and colonize contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

See [LICENSE][license-gpl] for details.

## Credits

The list of contributors to this project can be found at
[CONTRIBUTORS.md][contributors].

[contributing]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTING.md "Contribution guide"
[contributors]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTORS.md "List of contributors"
[license-gpl]: https://github.com/indiv0/colonize/blob/master/LICENSE-MIT "GPLv3 license"
