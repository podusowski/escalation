**This software is in very early development. There's really nothing to see here
for now.**

# What is it?

Escalation is a MMO space game, similar to
[Rusted](https://github.com/podusowski/rusted).

# Building and running tests

Just follow the typical Rust development workflow, i.e. `cargo build` , 
`cargo test` . Server's integration tests launch real application which uses
the logger so it might be beneficial to set `RUST_LOG` appropriately:

    cargo test --package esc_server

# Running server and client

Bevy [has a number of tricks][bevy-fast-compile] which makes the compilation
and the whole cycle faster. Most notable is using dynamic linking:

    RUST_LOG=info cargo run --features bevy/dynamic --bin esc_client

[bevy-fast-compile]: https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional

Server doesn't use bevy, therefore it can be started like any rust binary:

    RUST_LOG=info cargo run --bin esc_server

Alternatively, you can use script `./r.sh` to build and start both server and
client. When client is closed, server is terminated by the script as well.
