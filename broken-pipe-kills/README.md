# broken-pipe-kills

Kills your program instead of panicking with
```
thread 'main' (1964987) panicked at library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
```
due unhandled “Broken pipe” errors.

## Usage

Add

```toml
[dependencies]
broken-pipe-kills = "0.2.0"
```

to your `Cargo.toml` **and**

```rs
use broken_pipe_kills;
```

to `main.rs` to let `rustc` know it must be linked despite not being explicitly used.

This crate requires `nightly` Rust.

## Explanation And Examples

### The Problem

By default, the Rust standard library code [sets](https://github.com/rust-lang/rust/blob/1d05e3c131d7181eb3df1a8c261f43135c99200d/library/std/src/sys/pal/unix/mod.rs#L173) `SIGPIPE` to `SIG_IGN` before your `fn main()` runs. This converts `SIGPIPE` signals to `EPIPE` errors, which in turn is converted to [`std::io::ErrorKind::BrokenPipe`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.BrokenPipe).

When combined with `println!()` that [panics](https://github.com/rust-lang/rust/blob/1d05e3c131d7181eb3df1a8c261f43135c99200d/library/std/src/io/stdio.rs#L1165) upon errors, your program will crash when you e.g. pipe to `head`:

```rs
fn main() {
    loop {
        println!("hello world");
    }
}
```

```console
$ ./main | head
hello world
thread 'main' (1964987) panicked at library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
```

### The Solution

By using `broken-pipe-kills` (which overrides the Externally Implementable Item [`#[std::io::on_broken_pipe]`](https://github.com/rust-lang/rust/issues/150588)), `SIGPIPE` is set to `SIG_DFL` instead, which means your program is nicely killed and don't crash when e.g. piped to `head`:

```rs
use broken_pipe_kills;

fn main() {
    loop {
        println!("hello world");
    }
}
```

```console
$ ./main | head
hello world
```

### Audit the Code

This crate is tiny and easily audited with the following command [^1]:

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/broken-pipe-kills/0.3.0/download |
tar --extract --gzip --to-stdout |
less
```

[^1]: Please ensure you adhere to the [crates.io Data Access Policy](https://crates.io/data-access).
