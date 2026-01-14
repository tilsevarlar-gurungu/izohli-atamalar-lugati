# justfile

# Path to the Rust workspace directory
rust_dir := "mdbook-tiqmalarim"
# Path where Cargo outputs the compiled binaries
bin_dir := rust_dir + "/target/debug"

# Default recipe: List available commands
default:
    @just --list

# ==========================================
# 🦀 Rust / Plugin Management
# ==========================================

# Build the Rust workspace (all plugins)
build-rust:
    cargo build --manifest-path {{rust_dir}}/Cargo.toml

# Test the Rust workspace
test-rust:
    cargo test --manifest-path {{rust_dir}}/Cargo.toml

# Check code formatting (Rust)
fmt:
    cargo fmt --manifest-path {{rust_dir}}/Cargo.toml

# ==========================================
# 📚 mdBook Management
# ==========================================

# Build the book (Automatically builds Rust plugins first)
# We prepend the bin_dir to PATH so mdbook can find your custom preprocessors
build: build-rust
    export PATH="$PWD/{{bin_dir}}:$PATH" && mdbook build

# Serve on 0.0.0.0 to access from your phone/network
serve: build-rust
    @echo "IAL quyidagi manzilda: http://0.0.0.0:3000"
    export PATH="$PWD/{{bin_dir}}:$PATH" && mdbook serve -n 0.0.0.0

# ==========================================
# 🧹 Maintenance
# ==========================================

# Clean both Rust artifacts and mdbook build
clean:
    cargo clean --manifest-path {{rust_dir}}/Cargo.toml
    mdbook clean

# Run all tests (Rust tests + try to build the book to ensure integration works)
test-all: test-rust build
