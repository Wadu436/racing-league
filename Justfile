run_test_data:
    cargo run --bin cli race -f bahrain.bin -o bahrain.json
    cargo run --bin cli race -f las_vegas.bin -o las_vegas.json
    cargo run --bin cli race -f spa.bin -o spa.json


bahrain:
    cargo run --bin cli race -f bahrain.bin -o bahrain.json