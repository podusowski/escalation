set -e

function nuke() {
    echo "nuknig everything"
    for pid in $(jobs -p); do
        kill $pid
    done
}

RUST_LOG=debug cargo run -p esc_server &
trap nuke EXIT

RUST_LOG=esc_client=debug cargo run -p esc_client
