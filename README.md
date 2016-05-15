# colonize

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/indiv0/colonize" title="Travis Build Status"><img src="https://travis-ci.org/indiv0/colonize.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://indiv0.github.io/colonize/colonize" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/colonize" title="Crates.io"><img src="https://img.shields.io/crates/v/colonize.svg" alt="crates-io"></img></a>
            <a href="https://coveralls.io/github/indiv0/colonize?branch=master" title="Coverage Status"><img src="https://coveralls.io/repos/github/indiv0/colonize/badge.svg?branch=master" alt="coveralls-badge"></img></a>
        </td>
    </tr>
</table>

A Dwarf-Fortress/Rimworld-like game written in Rust.

***See the [changelog] for what's new in the most recent release.***

# Table of Contents

* [Running Precompiled Binaries](#running-precompiled-binaries)
* [Compiling & Running From Source](#compiling-and-running-from-source)
* [Configuration](#configuration)

## Running Precompiled Binaries

Pre-compiled binaries for each of the major targets can be found on the releases
page, [here][latest-release].

## Compiling & Running From Source
### Prerequisites

* [rust](https://www.rust-lang.org)

### Compiling

Compiling on Rustc stable:

```sh
cargo build
```

Compiling on Rustc nightly:

```sh
cargo build --no-default-features --features nightly
```

### Running

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

[changelog]: https://github.com/indiv0/colonize/blob/master/CHANGELOG.md
[colonize-json-example]: https://github.com/indiv0/colonize/blob/master/colonize.json.example "Example configuration"
[latest-release]: https://github.com/indiv0/colonize/releases/latest "Latest release"
