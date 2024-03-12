default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run 'cargo run' on the project
run *ARGS:
    cargo run -- {{ARGS}}

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
    cargo watch -x "run -- {{ARGS}}"

# Run 'cargo install' to install the binary in local bin
install *ARGS:
    cargo install --path ./ --root ~/.local/
