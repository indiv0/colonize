# colonize

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/indiv0/colonize" title="Travis Build Status"><img src="https://travis-ci.org/indiv0/colonize.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://indiv0.github.io/colonize/colonize" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt=license"></img>
            <a href="https://coveralls.io/github/indiv0/colonize?branch=master" title="Coverage Status"><img src="https://coveralls.io/repos/github/indiv0/colonize/badge.svg?branch=master" alt="coveralls-badge"></img></a>
        </td>
    </tr>
</table>

A Dwarf-Fortress/Rimworld-like game written in Rust.

***See the [changelog] for what's new in the most recent release.***

![colonize-screenshot](https://i.imgur.com/YI68SsY.jpg "Colonize - Game scene")

# Table of Contents

* [Running Precompiled Binaries](#running-precompiled-binaries)
* [Compiling & Running From Source](#compiling-and-running-from-source)
* [Configuration](#configuration)
* [Contributing](#contributing)
* [License](#license)

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

## Contributing

Contributions are always welcome!
If you have an idea for something to add (code, documentation, tests, examples,
etc.) fell free to give it a shot.

Please read [CONTRIBUTING.md][contributing] before you start contributing.

## License

Colonize is distributed under the terms of both the MIT license and the Apache
License (Version 2.0).

See [LICENSE-APACHE][license-apache], and [LICENSE-MIT][license-mit] for details.

## Credits

The list of contributors to this project can be found at
[CONTRIBUTORS.md][contributors].

[changelog]: https://github.com/indiv0/colonize/blob/master/CHANGELOG.md
[colonize-json-example]: https://github.com/indiv0/colonize/blob/master/colonize.json.example "Example configuration"
[contributing]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTING.md "Contribution guide"
[contributors]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTORS.md "List of contributors"
[latest-release]: https://github.com/indiv0/colonize/releases/latest "Latest release"
[license-apache]: https://github.com/indiv0/colonize/blob/master/LICENSE-APACHE "Apache-2.0 License"
[license-mit]: https://github.com/indiv0/colonize/blob/master/LICENSE-MIT "MIT License"
