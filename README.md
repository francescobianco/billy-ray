# billy-ray

`billy-ray` is a small Rust crate for embedding lightweight interactive terminal dialogues into command-line applications.

It is intended for tools that want a self-contained conversational prompt without mixing that logic into the main application flow. The crate handles a simple stdin/stdout exchange, keeps the interaction minimal, and is easy to call from an existing CLI binary.

## What it is for

This package is useful when you want:

- a compact interactive prompt for a CLI tool
- a reusable dialogue module separated from the main binary
- a tiny crate with no unnecessary application-level dependencies

The public API is intentionally small. The main entry point runs the dialogue and returns a standard I/O result, making it straightforward to integrate into command dispatch logic.

## Usage

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    billy_ray::run()?;
    Ok(())
}
```

## Design goals

- Keep the interaction self-contained
- Use plain standard input and output
- Stay simple enough to embed as a side feature in a larger CLI

## Note

Despite the sober packaging, this crate is, in fact, an easter egg.
