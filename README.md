# extabs
Implements expandtabs for String

![Crates.io](https://img.shields.io/crates/v/extabs?color=gree)
![Crates.io (recent)](https://img.shields.io/crates/dr/extabs?color=gree&label=downloads)

# Installation
Add this line in your Cargo.toml file
```toml
extabs = "1.0.2"
```

# Usage
```rust
fn main() {
    let s = String::from("Hello\tWorld!");
    let expanded = s.expandtabs(4);
    println!("{}", expanded);
}
```

