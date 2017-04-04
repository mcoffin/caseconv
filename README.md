# caseconv

`caseconv` is a simple library for easily converting strings (usually identifiers) between different case formats

# Building

```
cargo build --release
```

# Example

## Statically

```rust
use caseconv::{convert, case};
fn kebab_to_camel(kebab: &str) -> String {
    caseconv::convert(kebab, case::KEBAB, case::CAMEL)
}
```

## Dynamically

```rust
use caseconv::{convert, case, dynamic};
use caseconv::dynamic::CaseType;
fn jumbled_to_case(src: &str, dst_case: CaseType) -> String {
    caseconv::convert(src, dynamic::JUMBLED, &dst_case);
}
```

# License

`caseconv` is licenced under the `MIT License`.
