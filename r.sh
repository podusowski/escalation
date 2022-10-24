set -e

function nuke() {
    echo "nuknig everything"
    for pid in $(jobs -p); do
        kill $pid
    done
}

# Build everything first. Otherwise, server's compilation errors will go into
# the background.
cargo build --features bevy/dynamic

RUST_LOG=debug cargo run -p esc_server &
trap nuke EXIT

RUST_LOG=esc_client=debug cargo run --features bevy/dynamic --bin esc_client
