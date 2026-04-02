# billy-ray

`billy-ray` is a tiny Rust crate that runs a short terminal dialogue inspired by the "Billy Ray" gag from *Trading Places*.

It is designed for command-line tools that want to embed a lightweight interactive easter egg without pulling business logic into the main application.

## Usage

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    billy_ray::run()?;
    Ok(())
}
```
