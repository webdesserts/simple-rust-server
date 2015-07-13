Simple Rust Server
==================

Just a rust server that takes a url's query-string and sends it back as json.

Running
=======

If you have rust installed you should be able to do a quick `cargo run --release`
and it will build and run the server using the current `Cargo.lock`. This will
start a server listening on port 3000.

If you want to run the bin directly you can do...

```bash
cargo build --release
target/release/simple-rust-server
```
