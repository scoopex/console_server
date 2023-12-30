# A serial console server in rust

## Disclaimer

:warning: **This is my personal playground for playing with Rust. As always when learning a new programming language, the code is initially wrong, stupid, non-idomatic or even dangerous. But you can use this repository for your personal amusement :-)** :warning:


## Compile an Run


* Install Rust
* Install dependencies
  ```
  sudo apt-get install libacl1-dev
  cargo clean
  cargo build
  ```
* Run
  ```
  export RUST_LOG=DEBUG
  cargo run
  ```

## Create release

```
$ cargo build --release
$ du -sh  target/release/console_server
3,1M    target/release/console_server

```
