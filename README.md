# ColourSS

A Rust library for parsing CSS color strings into RGB values.

This project provides a library and a command-line tool to parse various
CSS color formats (Hex, RGB, HSL, Named) into a simple RGB struct.

## Installation

### From creates.io
The package is published on creates.io
```rust
cargo install colourss
```

### From source
```bash
git clone https://github.com/tumelonito/ColourSS.git
cd ColourSS
cargo build --release
```

## Technical Description

This library provides a single main function, `parse_color(input: &str)`,
which takes a string slice and attempts to parse it into a
`Color { r: u8, g: u8, b: u8 }` struct.

The parser works by trying to match the input string against a set of
predefined grammar rules. It checks in this order:

1.  Hex
2.  RGB/RGBA
3.  HSL/HSLA
4.  Named Color

If a match is found, it processes the string and returns the `Ok(Color)`.
If no rule matches, or if the format is invalid (e.g., wrong number of
components, bad numbers), it returns an `Err(ParseError)`.

### Grammar Rules

The parser understands the following formats:
<color> ::= <hex-color> | <rgb-color> | <hsl-color> | <named-color>
1.  **Hex:** `<hex-color> ::= '#__{3,4,6,8}__'`

      * `#rgb` (e.g., `#f03`)
      * `#rgba` (e.g., `#f03a`)
      * `#rrggbb` (e.g., `#ff0033`)
      * `#rrggbbaa` (e.g., `#ff0033aa`)

2.  **RGB(A):** `<rgb-color> ::= 'rgb(' <number> ',' <number> ',' <number> ')' | 'rgba(' ... ')'`

      * `rgb(255, 100, 0)`
      * `rgba(255, 100, 0, 0.5)`

3.  **HSL(A):** `<hsl-color> ::= 'hsl(' <hue> ',' <percent> ',' <percent> ')' | 'hsla(' ... ')'`

      * `hsl(120, 100%, 50%)`
      * `hsla(120, 100%, 50%, 1.0)`

4.  **Named:** `<named-color> ::= 'red' | 'blue' | ...`

      * `red`, `green`, `blue`, `white`, `black`, `yellow`, `rebeccapurple`
      * This is case-insensitive.

*(Note: For `rgba` and `hsla` formats, the alpha component is parsed
to ensure the format is valid, but it is discarded in the final `Color`
struct, as per the requirements.)*

### How to Use the Result

The resulting `Color` struct is a simple data container:

```rust
#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```
This struct can be used by any Rust application that needs to work with colors, such as:
* A game engine needing to set entity colors.
* A terminal application that wants to style its output.
* A web server templating engine that processes stylesheets.

### Command-Line Interface (CLI)
This project also includes a CLI app.
```rust
cargo run --help
```

### Commands:
```rust
cargo run --parse <path/to/file.txt>
```
This command will read the specified file and try to parse each line as a color. It will print the result for each line.

Example colors.txt:
#ff0000
blue
hsl(120, 100%, 50%)
not a color
rgb(10, 20, 30)


```rust
cargo run --credits
```
Displays author and license information.
