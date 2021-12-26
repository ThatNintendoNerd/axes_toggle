# Coordinate Axes Toggle
A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate designed to allow for the toggling of the free camera's coordinate axes figure.

The latest release can be found [here](https://github.com/ThatNintendoNerd/axes_toggle/releases/latest).

## Usage
By default, the visibility state of the coordinate axes figure is set to be disabled. Holding the Y button and pressing up or down on the D-pad will switch the visibility state:
- `Y & D-Pad Up` enables the figure and keeps it visible.
- `Y & D-Pad Down` disables the figure and keeps it invisible.

## Build from Source
If you would like to build the plugin starting from the source code, you can build the NRO file using the standard command for compiling Skyline plugins:

```
cargo skyline build --release
```

The resulting build is found under `/target/aarch64-skyline-switch/release/libaxes_toggle.nro`

### Prerequisites
- Stable [Rust](https://www.rust-lang.org/) environment
  - [cargo-skyline](https://github.com/jam1garner/cargo-skyline)

## Credits
* __jam1garner__ for the byte-searching code used in the plugin.
* __Raytwo__ for the offset-searching macro used in the plugin.
