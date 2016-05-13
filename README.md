# colonize [![Build Status](https://travis-ci.org/indiv0/colonize.svg?branch=master)](https://travis-ci.org/indiv0/colonize)

A Dwarf-Fortress/Rogue-like game written in Rust.

## Prerequisites

* [rust](https://www.rust-lang.org)
* [libtcod](http://roguecentral.org/doryen/libtcod/)

## Compiling

Compiling on Rustc stable:

```sh
cargo build
```

Compiling on Rustc nightly:

```sh
cargo build --no-default-features --features nightly
```

## Running

Running on Rustc stable:

```sh
cargo run
```

Running on Rustc nightly:

```sh
cargo run --no-default-features --features "nightly"
```

## Configuration

Currently, the `Config` struct holds all the configurable values for Colonize.
The struct is generated at compile time via a macro.

When running Colonize, the game first attempts to load the configuration from a
`colonize.json` file in the game's directory. If no such file is found, it falls
back to the specified defaults. An example `colonize.json` file can be found at
the root of this repo as [`colonize.json.example`][colonize-json-example].

In the future, the capability to define the config directory might be added.

[colonize-json-example]: https://github.com/indiv0/colonize/blob/master/colonize.json.example
