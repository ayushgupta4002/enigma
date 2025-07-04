# Enigma – Hide Messages Inside PNG Files

Enigma is a command-line utility written in Rust that lets you **embed, read, list, and remove arbitrary text messages inside the ancillary chunks of a PNG image**.  
The project is inspired by the *pngme* exercise from "The Rust Programming Language" and serves as a practical showcase of binary parsing, CRC validation, and ergonomic CLI design with **clap**.

---

## Features

* **Encode** – insert a custom chunk that contains your secret message.
* **Decode** – extract and print the hidden message from a given chunk.
* **Print**  – pretty-print every chunk that exists inside a PNG file.
* **Remove** – delete the first occurrence of a chunk of the requested type.

All chunks created by Enigma are fully-spec compliant – we calculate the CRC for you so that the resulting file remains a valid PNG image.

---

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) tool-chain (1.70+ recommended)

### Clone & Build

```bash
git clone https://github.com/ayushgupta4002/enigma.git
cd enigma
# build & run tests
cargo test
```

Because Enigma is a simple CLI, you can run it straight from `cargo` without installing:

```bash
cargo run -- --help  # note the double dash to forward args to the program
```

If you prefer a global binary:

```bash
cargo install --path .
```

---

## Usage

The binary exposes four sub-commands. Replace `enigma` with `cargo run --` when running from the workspace.

### 1. Encode

```bash
enigma encode <input.png> <chunk_type> "<message>" [output.png]
```

* `chunk_type` must be **four ASCII letters** (e.g. `ruSt`, `seCr`).  
* If `output.png` is omitted the input file will be modified in-place.

Example:

```bash
# Hide the text "The cake is a lie" inside a new ancillary chunk named ruSt
enigma encode home.png ruSt "The cake is a lie" output.png
```

### 2. Decode

```bash
enigma decode <png> <chunk_type>
```

```bash
enigma decode output.png ruSt
# Decoded message: The cake is a lie
```

### 3. Print

```bash
enigma print <png>
```

Lists every chunk (length, type, CRC) in human-readable form.

### 4. Remove

```bash
enigma remove <png> <chunk_type>
```

Removes the first matching chunk and rewrites the image.

---

## Internals

At the heart of Enigma are two core structures:

* `ChunkType` – validates the 4-byte identifier and exposes helper methods (`is_critical`, `is_public`, …).
* `Chunk`      – holds length, type, data, and CRC. Handles CRC calculation & validation.

`Png` glues everything together by iterating over the raw byte stream, parsing chunks, and providing safe helpers for mutation.  
Unit tests cover edge cases such as CRC mismatches and invalid headers so feel free to explore the source code in `src/`.

---

## Contributing

Bug reports and pull requests are welcome!  
If you have an idea for a new feature (e.g. AES encryption before embedding) open an issue first so we can discuss the scope.

To get started:

```bash
git checkout -b feat/your-feature
```

---

## License

This project is distributed under the MIT license. See `LICENSE` for more information. 