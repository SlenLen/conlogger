# per https://github.com/johnthagen/min-sized-rust
# not necessary, just use 'cargo build -r'

cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --release \
    && upx --best --lzma ./target/x86_64-unknown-linux-gnu/release/conlogger
