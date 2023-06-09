# Media Process Platform (MPP) module in rust FFI

Use c/cpp lib in rust.

Current supported modules:

+ rk_type
+ [ ] MPP
    + [x] mpp_log

[Acknowledge bindgen](https://rust-lang.github.io/rust-bindgen/)

[Reference document](https://raw.githubusercontent.com/rockchip-linux/mpp/develop/readme.txt)

## Directory Description

inc         : MPP header file

gen         : bindgen generated rust code from inc

etc.        : rust native code

## Usage

```shell
# Compile and install mpp to /usr/local/lib
git clone https://github.com/rockchip-linux/mpp.git

cd ./mpp/build/linux/to_your_arct/

./make-Makefiles.bash

make; sudo make install
```

```shell
# To generate ffi code, put c/cpp '.h' header in './inc/' first
cargo run ./utils/bindgen.rs

# or use bindgen cli
cargo install bindgen-cli
bindgen *.h -o ./gen/*.h.rs

# To use ffi, include .h*.rs code in `lib.rs`
echo 'include!(concat!("./gen/*.h.rs"));' >> src/lib.rs
# or manually modife and put in ./src/
```

## Notes

+ If you run cargo build into error and it say:

> = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified

You should compile native c/cpp code to binary lib, put them to somewhere like `/usr/local/bin`, and build again.

+ If you run cargo test into error and it say:

> = note: /usr/bin/ld: skipping incompatible

You should add target params like `cargo run --target=aarch64-unknown-linux-gnu`

+ If you run binary into error and it say:

> Caused by:
>
> Exec format error (os error 8)

Meanwhile, you cant run different architecture bin in your host machine.
