# on-broken-pipe

Experimental crates for the Externally Implementable Item [`#\[std::io::on_broken_pipe\]`](https://github.com/rust-lang/rust/issues/150588).

| Crate                                                                                          | Description                                                                               |
| ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| [`broken-pipe-kills`](https://github.com/Enselic/on-broken-pipe/tree/main/broken-pipe-kills)   | Kills your program instead of panicing when e.g. piping your output to `head`             |
| [`broken-pipe-errors`](https://github.com/Enselic/on-broken-pipe/tree/main/broken-pipe-errors) | Stops the Rust standard library from changing the SIPGIPE signal mask in child processes. |
| [`sigpipe-untouched`](https://github.com/Enselic/on-broken-pipe/tree/main/sigpipe-untouched)   | Makes the Rust standard library leave `SIGPIPE` completely untouched                      |
