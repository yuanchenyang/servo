make CFG_DISABLE_OPTIMIZE=1 CFG_ENABLE_DEBUG=1 && \
RUST_LOG=servo=4 ./servo -x demo.html 2>&1 |  ./filter
