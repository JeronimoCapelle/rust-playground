# rust-playground

This is a simple repository for tracking the small programs that I make while learning the rust programming language.

This repo only has tiny projects inside "src/bin", which you can run them individually by doing:

> `cargo run --bin 01_math_fn`

Im using "cargo watch", for QoL and enforcing proper standards.

>`cargo watch -c -q -x 'clippy -- -D warnings -W clippy::pedantic' -x 'run --bin 02_fn_parameters'`

---
***Warning***: the previous instructions suppose you have cargo installed through "rustup", and a proper linker like gcc or msvc.