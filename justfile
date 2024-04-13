flame test:
    cd grax-algorithms && cargo flamegraph --release --image-width 1920 --features extensive --unit-test grax_algorithms -- {{ test }}