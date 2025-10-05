# Last Stand \[prototype\]
[![Build Status](https://github.com/ivan-r-sigaev/last-stand-prototype/actions/workflows/rust.yml/badge.svg)](https://github.com/ivan-r-sigaev/last-stand-prototype/actions)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

This is a single-file web game created as a part of a learning project. 
The game is a bullet hell heavily inspired by [Brotato][1].
## Features
The goal of the project was to create a game that can can be distributed as a single self-contained HTML file.
- I used [`macroquad`][2] as the library for game development.
- I implemented ECS ([entity component component system][3]) with [sparse set compoenent pools][4].
- I implemented basic collision detection and optimized it using the [uniform collision grid][5].
- I baked assets into the executable with the [`include_bytes`][6] macro.
- I created a [script](./tools/build-wasm/build.py) to bundle the `.wasm` file into the [macroquad's HTML template][7] (*).

\* - This is needed to bypass the [CORS][8] when running the game without an http server (obviously, also because the game needs to be single-file).

## How to build from source
To build the project from source code you need to have the standard [rust toolchain](https://rust-lang.org/tools/install/) installed.

Clone the repository as follows or download and unpack the ZIP with the source code.
```bash
git clone https://github.com/ivan-r-sigaev/last-stand-prototype.git
cd last-stand-prototype
```

### Web assembly
Install the `wasm32-unknown-unknown` target for rustc (if not installed already).
```bash
rustup target add wasm32-unknown-unknown
```
Run the python build script.
```bash
cd tools/build-wasm
python3 ./build.py
```
The generated HTML file will be at `last-stand-prototype/target/last_stand.html`.
### Other targets
Run the [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) normally.
```bash
cargo build --release 
```

[1]: https://store.steampowered.com/app/1942280/Brotato/
[2]: https://macroquad.rs/
[3]: https://en.wikipedia.org/wiki/Entity_component_system
[4]: https://skypjack.github.io/2020-08-02-ecs-baf-part-9/
[5]: https://peerdh.com/blogs/programming-insights/efficient-grid-based-collision-detection-in-2d-games
[6]: https://doc.rust-lang.org/std/macro.include_bytes.html
[7]: https://mq.agical.se/release-web.html#create-an-html-page
[8]: https://en.wikipedia.org/wiki/Cross-origin_resource_sharing
