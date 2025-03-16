# Find Text Doc

[![crates.io](https://img.shields.io/crates/v/findtext_doc?label=latest)](https://crates.io/crates/findtext_doc)
[![Documentation](https://docs.rs/findtext_doc/badge.svg?version=latest)](https://docs.rs/findtext_doc)
[![Dependency Status](https://deps.rs/crate/findtext_doc/latest/status.svg)](https://deps.rs/crate/findtext_doc)
[![Releases Workflow](https://github.com/nabbisen/findtext-doc-rs/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/nabbisen/findtext-doc-rs/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/nabbisen/findtext-doc-rs)](https://github.com/nabbisen/findtext-doc-rs/blob/main/LICENSE)

## Summary

Search text in Document

## Development

First, add dependency:

```sh
cargo add findtext_doc
```

Usage:

```rust
use findtext_doc::search;

fn awesome_fn(keyword: &str, filepath: &str) {
    let ret = search(keyword, filepath);
}
```

## Executable Usage

Available in [Assets](https://github.com/nabbisen/findtext-doc-rs/releases/latest) in Releases. Cross-platform supported.

```sh
findtext_doc <keyword> <filepath>
# will print out like:
# Found.
# Missing.
```

## Acknowledgements

Depends on:

- [tafia/quick-xml](https://github.com/tafia/quick-xml) and [zip-rs/zip2](https://github.com/zip-rs/zip2)
