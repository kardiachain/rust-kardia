# protoc

## Build instruction
```
# you must install protoc
# ensure dependency protoc files are placed in /usr/local/include/
# workdir at kai-protos

# See: https://doc.rust-lang.org/cargo/reference/unstable.html
cargo +nightly build --out-dir=src -Z unstable-options
```