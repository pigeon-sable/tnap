# dgen
## Usage

```
Generate image with DALL·E and prnit it

Usage: dgen [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Prompt to pass to DALL·E

Options:
  -a, --ascii    Convert an image to ASCII art
  -h, --help     Print help
  -V, --version  Print version
```

```sh
$ cd dgen
$ cargo build
$ ./target/debug/dgen "prompt"
```
