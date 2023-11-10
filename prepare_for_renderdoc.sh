#/bin/sh
cargo build
rm bevy_snake
cp target/debug/bevy_snake .
