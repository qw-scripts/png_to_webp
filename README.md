# PNG to WebP Converter

A simple cli tool to convert PNG images to WebP format with customizable quality.

## Download

you can grab the pre-built binaries from the [releases](https://github.com/qw-scripts/png_to_webp/releases/latest) page.

## Usage

```bash
# Basic usage
png_to_webp -i input.png -o output.webp

# With custom quality (0-100, default is 75)
png_to_webp -i input.png -o output.webp -q 90

# With verbose output
png_to_webp -i input.png -o output.webp -v
```

## Building from source

if you want to build from source:

1. Make sure to have Rust installed, grab it from [here](https://rustup.rs/).
2. Clone the repository and navigate to the project directory.
3. Run `cargo build --release`.

The compiled binary will be available at `target/release/png_to_webp` or `target\release\png_to_webp.exe` depending on your platform.

## License

LGPL-3.0

## Contributing

anything is welcome, just open an issue or a pull request. I am sure there is a lot of room for improvement.

## Credits

-   [Github Action Template](https://github.com/marketplace/actions/build-and-upload-rust-binary-to-github-releases) for the CI/CD workflow.
