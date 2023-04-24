# BufPat • 


This pattern generation tool is used to create unique, non-repeating patterns that help identify buffer overflow vulnerabilities, allowing security researchers to pinpoint the precise location of a buffer overflow when exploiting or testing applications.

## Features

- Generate patterns of specified length
- Search for a given pattern within the generated pattern
- Find the offset of a given pattern within the generated pattern

## Installation

1. Install Rust using the instructions provided on the [official Rust website](https://www.rust-lang.org/tools/install).

2. Clone this repository:

```
git clone https://github.com/0xKoda/bufpat.git
cd bufpat
```

3. Build the project

```
cargo build --release
```


## Usage

After building the project, you can use the tool with the following commands:

- Generate a pattern of a specified length:

```
./target/release/bufpat <length>
```


Replace `<length>` with the desired pattern length.

- Find the offset of a given pattern within the generated pattern:
```
./target/release/bufpat <search_pattern>
```


Replace `<search_pattern>` with the pattern you want to search for.

## Examples

- Generate a pattern of length 20:

```
./target/release/bufpat 20
```

- Find the offset of the pattern "Aa5":
```
./target/release/bufpat Aa5
```

## License

This project is released under the [MIT License](LICENSE).
