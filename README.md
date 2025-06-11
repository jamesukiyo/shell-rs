# shell-rs 🦀

Basic shell written in Rust.

My second project written in Rust outside of learning/exercises.

If improvements can be made, please open a PR or issue! It's very far from
perfect but I hope to improve it as I learn and maybe use it frequently at some
point...

## Features

- Basic commands:
	- `ls`
	- `cd`
	- `pwd`
	- `help`
	- `exit`
- toml config file for the prompt (auto created) where you can change:
	- `emoji`
	- `display_name`
	- `separator`
	- `char`
	- `insert_blank_line`

See `default_config.toml`.

## Usage:

```
$ git clone https://github.com/jamesukiyo/shell-rs.git

$ cd shell-rs

$ cargo build --profile=release
```
Binary can be found at `./target/release/shell-rs`.

Run it:
```
$ ./target/release/shell-rs
```
A config file will be created at `~/.config/shell-rs/config.toml`.

## Changelog

- 0.1.0: base

## Improvements

- More commands - specifically would like to try `cat` and `grep`
- Options for commands - want to try `ls -a`, `ls -l` and combined
- Improve `ls` output
- Passthrough to system commands when not builtin
- Better support for invalid config (now it just fails to load with no indication)
- I don't think I handle errors very well so would like to improve that

## License

Copyright (c) James Plummer <jamesp2001@live.co.uk>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
