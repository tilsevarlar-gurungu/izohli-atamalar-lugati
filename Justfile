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
build: build-rust
    export PATH="$PWD/{{bin_dir}}:$PATH" && mdbook build

# Serve on 0.0.0.0 to access from your phone/network
serve: build-rust
    @echo "IAL quyidagi manzilda: http://0.0.0.0:3333"
    export PATH="$PWD/{{bin_dir}}:$PATH" && mdbook serve -n 0.0.0.0 -p 3333

# ==========================================
# ✍️ Terminology & Content
# ==========================================

# Yangi atama faylini yaratish (Masalan: just add-term clickbait)
add-term name:
    @mkdir -p src/terms
    @printf "# \n\n**Inglizça:** <br>\n**Rusça:** <br>\n**Ota-atama:** <br>\n**Qisqartma:** <br>\n**Sifat şakli:** <br>\n**Aloqali:** <br>\n**Soha:** <br>\n\n\n## Etiborga olingan muqobillar\n\n- " > src/terms/{{name}}.md
    @echo "Fayl yaratildi: src/terms/{{name}}.md"

# Latinga orqali imloni to'g'rilash (Masalan: just latinga src/terms/clickbait.md)
latinga path:
    latinga -n zahira/ial-qalqonlar.txt -u {{path}}

# ==========================================
# 🧹 Maintenance
# ==========================================

# Clean both Rust artifacts and mdbook build
clean:
    cargo clean --manifest-path {{rust_dir}}/Cargo.toml
    mdbook clean

# Run all tests (Rust tests + try to build the book to ensure integration works)
test-all: test-rust build

# ==========================================
# 📝 Eslatma
# ==========================================

eslatma:
    @echo 'just add-term open-source'
    @echo 'just latinga src/terms/open-source.md'
    @echo 'just urls'

# ==========================================
# 📝 Boğlamalar
# ==========================================

# Generates URLs for newly added markdown files
urls:
    #!/usr/bin/env bash
    BASE_URL="https://tilsevarlar-gurungu.github.io/izohli-atamalar-lugati/terms"

    # Fetch untracked files AND newly staged files, then filter for .md files
    NEW_FILES=$( { git ls-files --others --exclude-standard; git diff --cached --name-only --diff-filter=A; } | grep '\.md$' )

    if [ -z "$NEW_FILES" ]; then
        echo "No new markdown files found."
        exit 0
    fi

    # Loop through the files, extract the name, and print the formatted URL
    echo "$NEW_FILES" | while read -r file; do
        filename=$(basename "$file" .md)
        echo "$BASE_URL/${filename}.html"
    done
