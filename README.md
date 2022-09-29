**This software is in very early development. There's really nothing to see here
for now.**

# What is it?
Escalation is a MMO space game, similar to
[Rusted](https://github.com/podusowski/rusted).


# Building and running tests
Just follow the typical Rust development workflow, i.e. `cargo build`,
`cargo test`. Server's integration tests launch real application which uses
the logger so it might be beneficial to set `RUST_LOG` appropriately:

    RUST_LOG cargo test --package esc_server
