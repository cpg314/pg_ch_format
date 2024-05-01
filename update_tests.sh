set -euo pipefail
pushd tests
rm -r formatted || true
cp -r original formatted
cargo run -r -- -i formatted || true
cargo run -r -- formatted
popd
