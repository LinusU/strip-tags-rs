# Strip Tags

A port of the [`strip_tags` function from PHP](https://www.php.net/manual/en/function.strip-tags.php) to Rust.
Complete with a companion cli utility.

## Usage

### Crate

```rust
use strip_tags::strip_tags;

fn main() {
    let html = "<p>Hello, <strong>world</strong>!</p>";
    let text = strip_tags(html);
    assert_eq!(text, "Hello, world!");
}
```

See [documentation](https://docs.rs/strip-tags) for more information.


### Binary

The `strip-tags` util operates on stdin by default
```console
$ echo "<p>Hello, <strong>world</strong>!</p>" | strip-tags
Hello, world!

```

or a file may be specified instead
```console
$ strip-tags ./tests/hello.html
Hello, world!

```
