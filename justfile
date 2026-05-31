# -*- mode: just -*-

# Define build target
#
# NOTE: this can be changed with an environment variable
target := env("TARGET", "release")
build_option := if target != "debug" { "--" + target } else { "--" }
bin_name := `uvx --from=toml-cli toml get --toml-path=Cargo.toml package.name`

# Run the project
run:
    cargo run

# Build the project and copy and strip the resulting binary to the root project
[macos]
build: build-core
    cp -R target/dx/scratch/release/macos/Scratch.app ./

# Core build recipe using `cargo`, used by main build recipe
[private]
build-core: get-dioxus-cli
    dx build {{build_option}}

# Build the project for Windows
[macos]
build-win: rust-target-win
    # https://stackoverflow.com/a/62853319
    cargo build --target x86_64-pc-windows-gnu {{build_option}}

# Build the project for Windows
[linux]
build-win: get-mingw-w64
    # https://stackoverflow.com/a/62853319
    cargo build --target x86_64-pc-windows-gnu {{build_option}}

[private]
get-dioxus-cli:
    command -v dx || curl -fsSL https://dioxuslabs.com/install.sh | bash

[private, linux]
get-mingw-w64: rust-target-win
    dpkg -l | grep -qw mingw-w64 || sudo apt install -y mingw-w64

# Install required Rust toolchain for cross-compiling to Windows
[private, unix]
rust-target-win:
    # TODO: figure out requirements on BSD/macOS
    rustup target add x86_64-pc-windows-gnu

# Check project formatting and linting
fmt: pre-commit clippy cargo-fmt

[private]
pre-commit:
    uvx pre-commit run --all-files

[private]
cargo-fmt:
    cargo fmt --all -- --check

[private]
clippy:
    cargo clippy --all --all-targets --all-features -- --deny warnings --allow clippy::uninlined-format-args

# Update project dependencies in Cargo.lock
update:
    cargo update --locked --package {{bin_name}}

# Run tests
test:
    cargo test --all

# Generate doc
doc:
    cargo doc --open
