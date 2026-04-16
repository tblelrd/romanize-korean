# korean-romanize

`korean-romanize` is a Rust Library to romanize hangul (korean script) into
the latin alphabet.

This crate is very early in development, and I do not know how to
speak korean, so any help would be much appreciated! (I wrote this
crate to help romanize foreign lyrics my lyrics program.)

# Usage

To romanize:

```rust
assert_eq!(&korean_romanize::convert("안녕히 가세요"), "annyeonghi gaseyo");
```

To check if a string has korean characters:

```rust
assert!(korean_romanize::has_korean("안녕"));
assert!(korean_romanize::has_korean("안녕 Hello"));
assert!(!korean_romanize::has_korean("Hello"));
```

# Acknoledgements

This project could not have been started without:
- [kakasi](https://crates.io/crates/kakasi)
    - For the general structure of the project.
- [korean](https://crates.io/crates/korean)
    - For converting unicode to jana.
