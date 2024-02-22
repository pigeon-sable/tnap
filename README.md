# tnap - Let's take a nap 💤

## What's `tnap`?

`tnap` is the screen save for TUI.
You can use sample themes for tnap and generate image with default prompts or your own prompts.

## Examples

### Prompt

"high-contrast black and white Japanese anime illustration of a serene Japanese girl wearing cute headphones. She is depicted in a three-quarter view with her hand gently resting on one earpiece, enjoying the music. The image focuses on the texture of her hair and the soft expression on her face, all set against a pure black background to highlight the subject. Her hair is black and outline is white."

### Image

![girl_with_headphone.png](./examples/girl_with_headphone.png)

### Output

![girl_with_headphone_ascii.png](./examples/girl_with_headphone_ascii.png)

## Usage

```
You can use sample themes for tnap and generate image with default prompts or your own prompts

Usage: dgen [OPTIONS]

Options:
  -t, --theme <THEME>    Use the sample theme without generating images
  -k, --key <KEY>        Generate Image by looking up the corresponding value in config.toml using the subsequent string as a key and using it as a prompt
  -p, --prompt <PROMPT>  Generate images with user-considered prompt
  -a, --ascii            Convert an image to ASCII art
  -h, --help             Print help
  -V, --version          Print version
```

```sh
$ cargo build
$ ./target/debug/dgen "prompt"

or

$ cargo run -- "prompt"
```

## License

[Apache-2.0](./LICENSE)
