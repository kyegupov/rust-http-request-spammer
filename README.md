# rust-http-request-spammer

This is a port of https://gist.github.com/dmfed/dbc9f08df51ef8dd19c1dee94235aec6 to Rust

When spamming localhost on my laptop (default nginx page) using 100 "threads", I get 16K rps with the Go version and 90K rps with the Rust version.

Peak performance discovered: Go 25K RPS at 400 "threads", Rust 97K speed at 200 "threads".

To run:

- get Rust: https://rustup.rs/
- `cargo run --release`
