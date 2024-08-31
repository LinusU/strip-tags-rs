# Strip Tags

A port of the [`strip_tags` function from PHP](https://www.php.net/manual/en/function.strip-tags.php) to Rust.

## Usage

```rust
use strip_tags::strip_tags;

fn main() {
    let html = "<p>Hello, <strong>world</strong>!</p>";
    let text = strip_tags(html);
    assert_eq!(text, "Hello, world!");
}
```

## API

See [documentation](https://docs.rs/strip-tags) for more information.
