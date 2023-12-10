# per https://github.com/johnthagen/min-sized-rust
# not necessary, just use 'cargo build -r'
# I use this script to create the binary releases

RUSTFLAGS="-C opt-level="z"" \
cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --release \
    && upx --best --lzma ./target/x86_64-unknown-linux-gnu/release/conlogger

RUSTFLAGS="-C opt-level="z"" \
cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-pc-windows-gnu \
    --release \
    && upx --best --lzma ./target/x86_64-pc-windows-gnu/release/conlogger.exe
echo "Linux build at: $(cd "$(dirname "./target/x86_64-unknown-linux-gnu/release/conlogger")"; pwd)"
echo "Windows build at: $(cd "$(dirname "./target/x86_64-pc-windows-gnu/release/conlogger.exe")"; pwd)"
