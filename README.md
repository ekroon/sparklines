# Sparklines

A Rust library for Sparklines.

## Implementations

Sparklines exposes two indexer implementations:

- **Algorithmic** – computes the tick index using a simple formula.
- **Rangemap** – uses a range map to determine which tick to use.

```rust
use sparklines::{AlgorithmicSpark, RangemapSpark};

let a = AlgorithmicSpark::default();
let r = RangemapSpark::default();

assert_eq!(a.spark(&[1.0, 2.0, 3.0]), "▁▅█");
assert_eq!(r.spark(&[1.0, 2.0, 3.0]), "▁▄█");
```

## License
Dual licensed under MIT or APACHE 2.0 at your own choice.
