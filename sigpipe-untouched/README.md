# sigpipe-untouched

Makes the Rust standard library leave `SIGPIPE` completely untouched. In particular, this means `SIGPIPE` is
* untouched before `fn main()` runs
* untouched after [spawning](https://github.com/rust-lang/rust/blob/1d05e3c131d7181eb3df1a8c261f43135c99200d/library/std/src/sys/process/unix/unix.rs#L364) [child processes](https://github.com/rust-lang/rust/blob/1d05e3c131d7181eb3df1a8c261f43135c99200d/library/std/src/sys/process/unix/unix.rs#L737).

## Usage

Add

```toml
[dependencies]
sigpipe-untouched = "0.3.0"
```

to your `Cargo.toml` **and**

```rs
use sigpipe_untouched;
```

to `main.rs` to let `rustc` know it must be linked despite not being explicitly used.

This crate requires `nightly` Rust.
