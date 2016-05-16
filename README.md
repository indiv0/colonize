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

* [Platforms & Tool Chains](#platforms--tool-chains)
* [Running Precompiled Binaries](#running-precompiled-binaries)
* [Compiling & Running From Source](#compiling--running-from-source)
* [Configuration](#configuration)
* [Contributing](#contributing)
* [License](#license)

# Platforms & Tool Chains

`Colonize` should be compilable on any of the major rustc tool chains (stable, beta, or nightly).

In the long run, `Colonize` intends to support all major platforms (Windows/Mac OS X/Linux, 32-bit+64 bit). However, at the moment, I can only afford to prioritize one or two platforms at a time. For the rest, I will attempt to set up automated builds to at least ensure that the project compiles on the other platforms.

If you wish to help test or debug the game on any platform, please let me know! Your help would be greatly appreciated.

Both tool chain support and target support can be tracked at a glance via the [Travis CI](https://travis-ci.org/indiv0/colonize) page for the project.

Further information regarding the status of support for specific platforms can be found on the project's [issues](https://github.com/indiv0/colonize/issues) page.

A quick overview of the platforms and their status can be found below:

<table>
    <thead>
        <td>Target</td>
        <td>Confirmed Working</td>
        <td><a href="https://travis-ci.org/indiv0/colonize" title="Travis Build Status">Automated Travis/Appveyor Builds</a></td>
        <td><a href="https://github.com/indiv0/colonize/releases/latest" title="Latest release">Automated Deployment</a></td>
        <td>Notes</td>
    </thead>
    <tr>
        <td>i686-unknown-linux-gnu</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓ (missing assets)</td>
        <td>32-bit Linux (2.6.18+)</td>
    </tr>
    <tr>
        <td>x86_64-unknown-linux-gnu</td>
        <td>✓</td>
        <td>✓</td>
        <td>✓ (missing assets)</td>
        <td>64-bit Linux (2.6.18+)</td>
    </tr>
    <tr>
        <td>i686-pc-windows-gnu</td>
        <td></td>
        <td></td>
        <td></td>
        <td>32-bit MinGW (Windows 7+)</td>
    </tr>
    <tr>
        <td>x86_64-pc-windows-gnu</td>
        <td>✓</td>
        <td></td>
        <td></td>
        <td>64-bit MinGW (Windows 7+)</td>
    </tr>
    <tr>
        <td>i686-pc-windows-msvc</td>
        <td></td>
        <td></td>
        <td></td>
        <td>32-bit MSVC (Windows 7+)</td>
    </tr>
    <tr>
        <td>x86_64-pc-windows-msvc</td>
        <td></td>
        <td></td>
        <td></td>
        <td>64-bit MSVC (Windows 7+)</td>
    </tr>
    <tr>
        <td>i686-apple-darwin</td>
        <td></td>
        <td>(currently broken)</td>
        <td></td>
        <td>32-bit OSX (10.7+, Lion+)</td>
    </tr>
    <tr>
        <td>x86_64-apple-darwin</td>
        <td></td>
        <td></td>
        <td></td>
        <td>64-bit OSX (10.7+, Lion+)</td>
    </tr>
    <tr>
        <td>x86_64-unknown-linux-musl</td>
        <td></td>
        <td></td>
        <td></td>
        <td>64-bit Linux with MUSL</td>
    </tr>
</table>


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
cargo run --no-default-features --features nightly
```

## Configuration

Currently, the `Config` struct holds all the configurable values for Colonize.
This struct and further information on its usage can be found [here][config.in.rs].

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
[config.in.rs]: https://github.com/indiv0/colonize/blob/master/src/config.in.rs "config.in.rs"
[contributing]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTING.md "Contribution guide"
[contributors]: https://github.com/indiv0/colonize/blob/master/CONTRIBUTORS.md "List of contributors"
[latest-release]: https://github.com/indiv0/colonize/releases/latest "Latest release"
[license-apache]: https://github.com/indiv0/colonize/blob/master/LICENSE-APACHE "Apache-2.0 License"
[license-mit]: https://github.com/indiv0/colonize/blob/master/LICENSE-MIT "MIT License"
