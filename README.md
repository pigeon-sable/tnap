# dgen
## Usage

```
Generate an image with DALL·E and display it on the terminal

Usage: dgen [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Prompt to pass to DALL·E

Options:
  -a, --ascii    Convert an image to ASCII art
  -h, --help     Print help
  -V, --version  Print version
```

```sh
$ cargo build
$ ./target/debug/dgen "prompt"

or 

$ cargo run -- "prompt"
```

## License
[Apache-2.0](./LICENSE)
