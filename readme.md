# mdr

## Disclaimer

If the content of the readme is subject to frequent weird changes,
it's because I'm just testing the parser on it.

## Description

*mdr*, which stands for markdown renderer, is a simple markdown parser written in Rust.
It supports basic markdown syntax and follows the [CommonMark Specs](https://spec.commonmark.org/0.31.2/) (0.31.2).

I have made this project mostly to learn the Rust programming language.
I am not trying to bring some kind of innovations or new optimized ways to parse markdown.
So if you are expecting that, there are probably better tools out there.

## Current Features

This tool can parse the following markdown elements:

- Headings
- Paragraphs

## Future Features

- CLI arguments to pick a preferred markdown flavor (maybe)
- Support for more markdown elements

## Installation

For now, to install mdr, you need to clone the repository and build it manually:

```bash
git clone https://github.com/yourusername/mdr.git
cd mdr
cargo build --release
```

## Usage

To use mdr, run the following command:

```bash
mdr <input_file> (output_file)
```
