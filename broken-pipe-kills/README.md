# broken-pipe-kills

## What is the problem?

Normally the Rust standard library code sets `SIGPIPE` to `SIG_IGN` before your `fn main()` runs. This makes you see an error if you pipe your output to something like `head`:

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
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', library/std/src/io/stdio.rs:1016:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
```

## How do I solve it?

Add the following to your `Cargo.toml` to set `SIGPIPE` to `SIG_DFL` instead so your program is nicely killed instead of panicking:

```toml
[dependencies]
broken-pipe-kills = "0.2.0"
```

```console
$ ./main | head
hello world
```

### Audit the Code

This crate is tiny and easily audited with the following command [^1]:

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/broken-pipe-kills/0.2.0/download |
tar --extract --gzip --to-stdout |
less
```

[^1]: Please also see [crates.io Data Access Policy](https://crates.io/data-access).

## When Can I Use It?

Follow [Tracking Issue for Externally Implementable Item `#[std::io::on_broken_pipe]` #150588](https://github.com/rust-lang/rust/issues/150588) for updates.
