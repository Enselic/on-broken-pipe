# on-broken-pipe

Experimental crates for the Externally Implementable Item [`#\[std::io::on_broken_pipe\]`](https://github.com/rust-lang/rust/issues/150588).

| Crate                | Description                                                                   |
| -------------------- | ----------------------------------------------------------------------------- |
| `broken-pipe-kills`  | Kills your program instead of panicing when e.g. piping your output to `head` |
| `sigpipe-untouched`  | Makes the Rust standard library leave `SIGPIPE` completely untouched          |
