# tinyeasings

**[Interactive Rust demo](https://varnholt.github.io/tinyeasings/)** — [run it natively or build for the browser](https://github.com/varnholt/tinyeasings/blob/rust-v0.1.1/rust/demo/README.md).

All 30 easings.net curves for `f32` and `f64`, with runtime curve selection. Native Rust, no dependencies, no unsafe code. CC0-1.0, matching the C++ header. Requires Rust 1.85 or newer.

```rust
use tinyeasings::{ease, ease_out_bounce, Curve};

assert_eq!(ease_out_bounce(0.5_f64), 0.765625);
let value: f32 = ease(Curve::InOutCubic, 0.5);
assert_eq!(value, 0.5);

const QUARTER: f64 = tinyeasings::f64::ease_in_quad(0.5);
assert_eq!(QUARTER, 0.25);
```

## Use

```toml
[dependencies]
tinyeasings = "0.1"
```

The crate lives alongside the C++ implementation in `rust/`. To use a local checkout, replace the version with `{ path = "/path/to/tinyeasings/rust" }`.

[crates.io](https://crates.io/crates/tinyeasings) | [API documentation](https://docs.rs/tinyeasings)

## API

The generic top-level functions accept either `f32` or `f64`:

- `ease_in_sine`, `ease_out_sine`, `ease_in_out_sine`
- the same three forms for `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` and `bounce`

`ease(curve, x)` and `curve.sample(x)` select at runtime. `ALL_CURVES` contains all 30 variants, `CURVE_COUNT` gives the count, and `curve.name()` / `name(curve)` return the C++ names, such as `easeOutBounce`. `Curve::try_from(u8)` rejects unknown numeric IDs; variant order matches the C++ enum.

The explicit `tinyeasings::f32` and `tinyeasings::f64` modules expose the same functions. Polynomial, back and bounce functions in these modules are `const fn`; transcendental functions and the generic dispatch are evaluated at runtime.

Endpoints are exactly 0 and 1. Inputs are not clamped; back and elastic curves intentionally overshoot. Inputs outside the usual domain may produce values outside [0, 1] or NaN. Functions use the standard library's floating-point math; this is not a `no_std` crate.

## Check and run

Run from `rust/`:

```sh
cargo test
cargo run --example curves
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Tests check endpoints, symmetry, precision, runtime IDs, overshoot and compile-time polynomial evaluation. C++ fixtures compare every curve at 101 points, in both precisions. To regenerate them, run `python tools/generate_reference.py` with a C++23 compiler (`CXX` selects it). `--check` verifies stored samples on the same toolchain. Normal Cargo builds do not need C++.

CI checks native platforms, Rust 1.85, WebAssembly, packaging and freshly generated C++ reference samples.
