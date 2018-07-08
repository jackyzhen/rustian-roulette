# :gun: Rustian Roulette :gun:

_*Warning:*_ *Play at your own risk.*

Have you ever wondered to yourself: "Why do I need files?".

Maybe you don't need files. Atleast, maybe you don't need _all_ your files.

Why not play *Rustian Roulette* and let the RNG Gods decide what files are unworthy of continued existence.

## Features

- By default, deletes one random file or directory in your home directory with 1/6th probability.
- Specify root path and 1/x probability.
- Built with rust, so you know your files will be deleted with guaranteed memory safety, thread safety and efficiently with zero-cost abstractions :rocket: :rocket: :rocket:

## Usage

Install and run via [cargo](https://crates.io/crates/rustian-roulette):

```
cargo install rustian-roulette

```

Run:
```
~ rr
Path = /Users/jackyzhen, Chances = 1/6 (16.67%)
If you are unlucky, this will delete random file(s) in above path. Are you sure you want to play? <Y,y,YES,yes>: y

✝💀/Users/jackyzhen/.bashrc💀✝ *BANG*
```

Prebuild binaries for all platforms coming soon (maybe).

## Notes:

Files were most certainly harmed in the making of this existential experiment.

"Inspired" by [rmraf](https://github.com/tomarrell/rmraf).

## License:

This project is licensed under [GLWTPL](https://github.com/me-shaon/GLWTPL/blob/master/LICENSE).
