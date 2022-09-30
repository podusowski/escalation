**This software is in very early development. There's really nothing to see here
for now.**

# What is it?

Escalation is a MMO space game, similar to
[Rusted](https://github.com/podusowski/rusted).

# Building and running tests

Just follow the typical Rust development workflow, i.e. `cargo build` , 
`cargo test` . Server's integration tests launch real application which uses
the logger so it might be beneficial to set `RUST_LOG` appropriately:

    RUST_LOG cargo test --package esc_server

# Running the client

Bevy [has a number of tricks][bevy-fast-compile] which makes the compilation
and the whole cycle faster. Most notable is using dynamic linking:

    RUST_LOG=info cargo run --features bevy/dynamic --bin esc_client

[bevy-fast-compile]: https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional
