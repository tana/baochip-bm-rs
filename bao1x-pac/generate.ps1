if (Test-Path src) {
    Remove-Item -Recurse src
}

svd2rust -c .\svd2rust-config.toml

form -i lib.rs -o src/
cargo fmt
