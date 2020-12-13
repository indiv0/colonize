# colonize

A Dwarf Fortress/Rimworld-like game written in Rust.

<table>
  <tr>
    <td><strong>Live Demo (v0.3.0; <a href="https://github.com/indiv0/colonize#controls">Controls</a>)</strong></td>
    <td><a href="https://colonize.rs/0.3.0/">https://colonize.rs/0.3.0/</td>
  </tr>
  <tr>
    <td colspan="2">
      <img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg" alt="license"></img>
    </td>
  </tr>
</table>

***See the [changelog] for what's new in the most recent release.***

![colonize-screenshot](https://colonize.rs/colonize-v0.3.0.png "Colonize v0.3.0 - Game scene")
Colonize v0.3.0 terrain. Source: [Carter Anderson](https://github.com/cart)

# Table of Contents

* [About](#about)
* [Controls](#controls)
* [Compiling & Running From Source](#compiling--running-from-source)
* [Contributing](#contributing)
* [License](#license)

# About

I've always wanted to create a game like Dwarf Fortress or Rimworld.
A game with emergent complexity, engrossing simulation, and the potential for _fun_ that comes from that.

This is my attempt to build that game.

My view on what exactly this game should be like is unclear, and at the moment
it's mostly a technical experiment and personal exploration of game development.

The best way I can summarize my current vision is that this game should provide
a real-time simulation of a world in which individual entities (like dwarves in
Dwarf Fortress) perform actions to satisfy goals set by the player (e.g. "build a house",
"harvest some wood", "mine some stone"). The gameplay will focus on getting a
player to build a fort/base and protect it from threats. These threats may be
the elements, monsters, or various catastrophes. For now, the game is intended to
be single-player only.

Currently, the game only generates some basic 3D voxel terrain from fractal brownian motion.
My next step is probably to add some basic AI entities ("dwarves") and have them walk
around the map and harvest some resources.

# Controls

The controls are a bit wonky right now, and they vary depending on whether you're
playing the game in the browser (WASM) or on your machine (native).

Currently, the controls are focused on moving around the map and regenerating
the terrain with new parameters.

- Movement: W/A/S/D/Q/E
- Orientation: Arrow Keys or Numpad Arrows (WASM); Mouse (Native)
- Terrain:
    - Y-Offset (+/- 1): U/H
    - Frequency (+/- 0.001): I/J
    - Lacunarity (+/- 0.1): O/K
    - Persistence (+/- 0.1): P/L

# Compiling & Running From Source
## Prerequisites

* [rust](https://www.rust-lang.org)

## Enable Fast Compiles (Optional)

For fast iterative compiles, follow the [instructions in the Bevy book](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)
on how to setup fast compiles.

## Compiling

Build & run:
```sh
cargo run --release
```

## Compiling for WASM

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

# Contributing

Contributions are always welcome!
If you have an idea for something to add (code, documentation, tests, examples,
etc.) fell free to give it a shot.

Please read [CONTRIBUTING.md][contributing] before you start contributing.

# License

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

# Credits

This project is possible due to the excellent work of many developers.
Check out the some of the giants whose shoulders we stand on:
* [Bevy](https://bevyengine.org/) - Game engine
* [bonsairobo/building-blocks](https://github.com/bonsairobo/building-blocks) - Voxel library
* [mrk-its/bevy_webgl2](https://github.com/mrk-its/bevy_webgl2) - WebGL2 renderer plugin
* [aevyrie/bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking) - 3D mouse picking plugins

The list of contributors to this project can be found at
[CONTRIBUTORS.md][contributors].

[changelog]: https://github.com/indiv0/colonize/blob/master/CHANGELOG.md "Changelog"
[contributing]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTING.md "Contribution guide"
[contributors]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTORS.md "List of contributors"
[license-gpl]: https://github.com/indiv0/colonize/blob/master/LICENSE-MIT "GPLv3 license"
