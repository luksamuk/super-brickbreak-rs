# SUPER BRICKBREAK - WebAssembly Edition
by Lucas Vieira (luksamuk)<br/>
Copyright (c) 2018 Lucas Vieira.

**NOTE: This is a direct port of Super BrickBreak. For the original code -- and for actually playing the game --, check out the [main JavaScript version](https://github.com/luksamuk/SuperBrickBreak).**

Fast-paced arkanoid featuring 20+ levels.

*(I plan to add a screenshot here as soon as it is feasible, I swear.)*


## About
This game is a Rust + WebAssembly port of its JavaScript counterpart, which is also a direct port of its C++ counterpart (found in my MiniGames repo), and part of a collection of mini-games I made in the past.
This game was originally written in Processing (Java), ported to C++ (OficinaFramework v1.3/OpenGL 3.1), completely ported to JavaScript using the 2D canvas API, and now, ported to Rust and with most of its logic running under the brand new WebAssembly standard.
This game is still a very rough prototype and highly incomplete compared to its JavaScript counterpart. You can [play it on your browser](https://luksamuk.github.io/super-brickbreak-rs), if you want to give it a spin.

## Goal
The goal of this port is to make full use of WebAssembly and, while it still needs JavaScript interop for some features missing in the `stdweb` crate, it runs very smoothly.

## Compiling

You *need* Rust Nightly installed in order to build this -- and not only Nightly, but specifically `nightly-2018-01-21`. You can install it by using

	rustup toolchain install nightly-2018-01-21
	rustup target add wasm32-unknown-unknown --toolchain nightly

You'll also need `cargo-web` >= 0.6.8:

	cargo install cargo-web

Also, if you're not using `rustup`, please do, before it's too late for your soul.

## Disclaimer and Special Notes
Please don't be a douche; do not deliberately steal this code.
You can use the code as a reference for your own game, as long as you respect the [license](./LICENSE).
This game uses [GohuFont](font.gohu.org) by Hugo Chargois, ported to TTF by [Guilherme Maeda](https://github.com/koemaeda/gohufont-ttf).

## To-Do List
- Basic JavaScript version -- 60%
- Heads-up display -- 20%
- Improve collision detection -- 40%
- Improve overall appearance -- 10%
- Distribute code in a better way -- 50%

