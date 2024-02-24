<div align="center">

# tnap - Let's take a nap 💤

![Demo](./examples/demo.gif)

`tnap` is a screen saver for the terminal.
You can rest the terminal in a secure.

[![Lang](https://img.shields.io/badge/Rust-1.26+-blue.svg?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![Release](https://img.shields.io/badge/Release-v0.1.0-blue.svg)

</div>

## Features

- [x] Display images in the terminal and use it as a screen saver
- [x] Convert images to **_ASCII art_**
- [x] Use images generated using **_[DALL-E 3](https://openai.com/dall-e-3)_**
- [x] Generate by simply specifying a key in `config.toml` without thinking a prompt
- [x] Of course, you can also generate images by specifying a prompt

## Screenshots

<img src="./examples/girl_with_headphone.png" alt="girl_with_headphone" width="45%">
<img src="./examples/girl_with_headphone_ascii.png" alt="girl_with_headphone_ascii" width="45%">

## Usage

| Option              | Description                                          | Type   | Required? |
| ------------------- | ---------------------------------------------------- | ------ | --------- |
| `--theme <THEME>`   | Use the sample theme without generating images       | String | Yes       |
| `--key <KEY>`       | Generate images by a default prompt in `config.toml` | String | Yes       |
| `--prompt <PROMPT>` | Generate images with a user's prompt                 | String | Yes       |
| `--ascii`           | Convert an image to ASCII art                        | bool   | Yes       |

## Installation

You can install tnap using Homebrew and cargo:

```bash
brew install tnap
```

```bash
cargo install tnap
```

## Acknowledgements

- [sheepla/pingu](https://github.com/sheepla/pingu) - 🐧ping command but with pingu
- [mtoyoda/sl](https://github.com/mtoyoda/sl) - SL(1): Cure your bad habit of mistyping
- [dduan/tre](https://github.com/dduan/tre) - Tree command, improved.
- [dalance/procs](https://github.com/dalance/procs) - A modern replacement for ps written in Rust
- [sharkdp/bat](https://github.com/sharkdp/bat) - A `cat` clone with wings.
- [ogham/exa](https://github.com/ogham/exa) - A modern replacement for ‘`ls`.

## License

🪪 [Apache-2.0](./LICENSE)

## Authors

- 🍪 [@shuheykoyama](https://github.com/shuheykoyama)
- 🦀 [@4n12i](https://github.com/4n12i)
- 👮 [@Kobayashi123](https://github.com/Kobayashi123)
- 🧪 [@Gteruya](https://github.com/Gteruya)

[![GitHub](https://img.shields.io/badge/-Follow--FFFFFF?style=social&logo=github&label=Follow%20pigeon-sable)](https://github.com/pigeon-sable)
